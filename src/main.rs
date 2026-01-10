extern crate rust_gpiozero as gpio;
extern crate dialoguer;

use gpio::output_devices::Servo;

mod indicator_led;
//use indicator_led::{IndicatorLED, IndicatorLEDState};
mod digipot;
use digipot::Digipot;

use crate::indicator_led::{IndicatorLED, SafetyState};

const DIGIPOT_CS: u8 = 1;
const DIGIPIT_INC: u8 = 1;
const DIGIPOT_UD: u8 = 1;
const RED_LED_PIN: u8 = 1;
const SERVO_PIN: u8 = 1;

#[macro_export]
macro_rules! delay_ms {
    ($e:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($e))
    };
}

fn main() {
    let mut motor = Digipot::new(DIGIPOT_CS, DIGIPIT_INC, DIGIPOT_UD, 100);
    let mut red_led = IndicatorLED::new(RED_LED_PIN);
    let mut servo = Servo::new(SERVO_PIN);
    servo.set_position(1.0);
    red_led.set(SafetyState::Safe);
    motor.set(100);
    dialoguer::Password::default()
        .with_prompt("Press Enter to stop.")
        .interact()
        .unwrap();
}
