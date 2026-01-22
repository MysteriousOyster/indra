extern crate crossterm;
extern crate rppal;
extern crate serde;
extern crate toml;
extern crate libcamera;

mod indicator_led;
mod servo;
//use indicator_led::{IndicatorLED, IndicatorLEDState};
mod digipot;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use libcamera::camera_manager::CameraManager;
use rppal::{gpio::Gpio, pwm::Channel};
use serde::Deserialize;
use std::{
    fs, io,
    path::Path,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

use crate::{
    digipot::Digipot,
    indicator_led::{IndicatorLED, SafetyState},
    servo::Servo,
};

const DIGIPOT_CS: u8 = 11;
const DIGIPOT_INC: u8 = 27;
const DIGIPOT_UD: u8 = 22;
const RED_LED_PIN: u8 = 26;

#[derive(Deserialize)]
struct ProgramConfig {
    picture_dir: String,
    num_pictures: usize,
    motor_step_off: u8,
    motor_step_slow: u8,
    motor_step_on: u8,
    servo_retract_percent: f64,
    servo_extend_percent: f64,
    delay_retract_slow: u64,
    delay_extend_slow: u64,
    delay_wind: u64,
    delay_picture: u64,
}

fn take_picture(path: &Path) -> io::Result<Child> {
    Command::new("rpicam-still")
        // Set the path of the image
        .arg("-o")
        .arg(path.as_os_str())
        // Disable preview
        .arg("--nopreview")
        // Set highest JPEG quality
        .arg("--quality")
        .arg("100")
        // Set the '''best''' mode settings
        .arg("--mode")
        .arg("3280:2464:12:P")
        .stdout(Stdio::null())
        .spawn()
}

fn main() -> io::Result<()> {
    // Configuration loading
    let filename = "conf.toml";
    let contents = fs::read_to_string(filename)?;
    let config: ProgramConfig = toml::from_str(&contents).expect("conf.toml incorrectly formatted");
    let picture_dir = Path::new(config.picture_dir.as_str());

    let cm = CameraManager::new().expect("failed to start camera manager");
    

    let gpio = Gpio::new().unwrap();
    let mut motor = Digipot::new(
        gpio.get(DIGIPOT_CS)
            .expect("failed to get CS pin")
            .into_output(),
        gpio.get(DIGIPOT_INC)
            .expect("failed to get INC pin")
            .into_output(),
        gpio.get(DIGIPOT_UD)
            .expect("failed to get UD pin")
            .into_output(),
        100,
    );
    let mut red_led = IndicatorLED::new(
        gpio.get(RED_LED_PIN)
            .expect("failed to get red LED pin")
            .into_output(),
    );
    let servo = Servo::new(Channel::Pwm0).expect("failed to get servo");
    servo.set_percent(config.servo_retract_percent).unwrap();
    red_led.set(SafetyState::Safe).unwrap();
    motor.set(config.motor_step_off);
    println!("Press ENTER to stop.");
    let mut picture_counter: u32 = 1;
    enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Enter {
                    println!("Breaking.");
                    break;
                }
            }
        }

        // Retract Servo
        servo.set_percent(config.servo_retract_percent).unwrap();
        // Turn motor off
        motor.set(config.motor_step_off);
        // Wait for servo & motor
        thread::sleep(Duration::from_millis(config.delay_retract_slow));
        // Push servo in and turn motor to slow
        servo.set_percent(config.servo_extend_percent).unwrap();
        motor.set(config.motor_step_slow);
        // Let's wait for the Servo to go in.
        thread::sleep(Duration::from_millis(config.delay_extend_slow));
        // We are dealing with dangerous tension now, so we should indicate it.
        red_led.set(SafetyState::Unsafe).unwrap();
        // Now, we will ramp up the speed of the motor.
        motor.set(config.motor_step_on);
        // Wait for it to wind up.
        thread::sleep(Duration::from_millis(config.delay_wind));
        // Ok, now we should be good to retract the servo and cut the motor.
        motor.set(config.motor_step_off);
        servo.set_percent(config.servo_retract_percent).unwrap();
        // Let's take a few pictures.
        let mut last_handle: Option<Child> = None;
        for i in 1..=config.num_pictures {
            let picture_name = format!("{picture_counter}-{i}.jpeg");
            let picture_path = picture_dir.join(picture_name);
            if let Some(ref mut x) = last_handle {
                if let Ok(Some(_)) = x.try_wait() {
                    last_handle = Some(take_picture(picture_path.as_path()).unwrap());
                }
            } else {
                last_handle = Some(take_picture(picture_path.as_path()).unwrap());
            }
            thread::sleep(Duration::from_millis(config.delay_picture));
        }
        red_led.set(SafetyState::Safe).unwrap();
        picture_counter += 1;
    }
    disable_raw_mode()
}
