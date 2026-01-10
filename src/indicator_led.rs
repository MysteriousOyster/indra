extern crate rust_gpiozero as gpio;

use std::ops::{Deref, DerefMut};

use gpio::{LED};

pub enum SafetyState {
    Sleep,
    Safe,
    Unsafe
}

pub struct IndicatorLED {
    output: LED,
}

impl IndicatorLED {
    pub fn new(pin: u8) -> Self {
        let output = LED::new(pin);
        output.on();
        Self {
            output
        }
    }

    pub fn set(&mut self, state: SafetyState) {
        match state {
            SafetyState::Sleep => self.on(),
            SafetyState::Unsafe => self.blink(0.5, 0.5),
            SafetyState::Safe => self.blink(1.0, 2.0),
        }
    }
}

impl Deref for IndicatorLED {
    type Target = LED;
    fn deref(&self) -> &Self::Target {
        &self.output
    }
}

impl DerefMut for IndicatorLED {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.output
    }
}

impl Drop for IndicatorLED {
    fn drop(&mut self) {
        self.off();
    }
}