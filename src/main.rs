extern crate rppal;
extern crate crossterm;

mod indicator_led;
mod servo;
//use indicator_led::{IndicatorLED, IndicatorLEDState};
mod digipot;
use crossterm::{event::{self, Event, KeyCode}, terminal::{disable_raw_mode, enable_raw_mode}};
use rppal::{gpio::Gpio, pwm::Channel};
use std::{
    io,
    process::{Child, Command}, thread, time::Duration,
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

fn take_picture(path: &str) -> io::Result<Child> {
    Command::new("rpicam-still")
        // Set the path of the image
        .arg("-o")
        .arg(path)
        // Disable preview
        .arg("--nopreview")
        // Set highest JPEG quality
        .arg("--quality")
        .arg("100")
        // Set the '''best''' mode settings
        .arg("--mode")
        .arg("3280:2464:12:P")
        .spawn()
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
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
    servo.set_percent(100.0).unwrap();
    red_led.set(SafetyState::Safe).unwrap();
    motor.set(50);
    println!("Press ENTER to stop.");
    let mut picture_counter: u32 = 1;
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
        servo.set_percent(0.0).unwrap();
        // Set motor to slow speed
        motor.set(80);
        // Wait for servo & motor 
        thread::sleep(Duration::from_millis(200));
        // Push servo in
        servo.set_percent(100.0).unwrap();
        // Let's wait for the Servo to go in.
        thread::sleep(Duration::from_millis(2000));
        // We are dealing with dangerous tension now, so we should indicate it.
        red_led.set(SafetyState::Unsafe).unwrap();
        // Now, we will ramp up the speed of the motor.
        motor.set(0);
        // Wait for it to wind up.
        thread::sleep(Duration::from_millis(5000));
        // Ok, now we should be good to retract the servo and cut the motor.
        motor.set(100);
        servo.set_percent(0.0).unwrap();
        // Let's take a few pictures.
        for i in 1..=5 {
            let filename = format!("~/pictures/{picture_counter}-{i}.jpeg");
            take_picture(filename.as_str()).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
        red_led.set(SafetyState::Safe).unwrap();
        picture_counter += 1;
    }
    disable_raw_mode()
}
