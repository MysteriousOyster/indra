extern crate dialoguer;
extern crate rppal;

mod indicator_led;
//use indicator_led::{IndicatorLED, IndicatorLEDState};
mod digipot;
use digipot::Digipot;
use rppal::gpio::Gpio;

use crate::indicator_led::{IndicatorLED, SafetyState};

const DIGIPOT_CS: u8 = 4;
const DIGIPIT_INC: u8 = 5;
const DIGIPOT_UD: u8 = 6;
const RED_LED_PIN: u8 = 4;
const SERVO_PIN: u8 = 5;

#[macro_export]
macro_rules! delay_ms {
    ($e:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($e));
    };
}
#[macro_export]
macro_rules! future_sec_f64 {
    ($e:expr) => {
        std::time::Instant::now() + std::time::Duration::from_secs_f64($e)
    };
}

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut motor = Digipot::new(
        gpio.get(DIGIPOT_CS).unwrap().into_output_high(),
        gpio.get(DIGIPIT_INC).unwrap().into_output_high(),
        gpio.get(DIGIPOT_UD).unwrap().into_output_high(),
        100,
    );
    // let mut red_led = IndicatorLED::new(gpio.get(RED_LED_PIN).unwrap().into_output_high());
    motor.set(100);
    dialoguer::Password::default()
        .with_prompt("Press Enter to stop.")
        .interact()
        .unwrap();
}
