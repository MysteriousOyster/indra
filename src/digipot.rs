extern crate rppal;
use std::cmp::Ordering;

use rppal::gpio::OutputPin;

use super::delay_ms;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

pub struct Digipot {
    /// High -> disabled, Low -> enabled.
    cs: OutputPin,
    /// Low -> trigger
    inc: OutputPin,
    // High -> up, Low -> down
    ud: OutputPin,
    // From 1 -> steps
    steps: u8,
    current_step: Option<u8>,
}

impl Digipot {
    pub fn new(cs: OutputPin, inc: OutputPin, ud: OutputPin, steps: u8) -> Self {
        Digipot {
            cs,
            inc,
            ud,
            steps,
            current_step: None,
        }
    }
    pub fn wipe(&mut self, steps: u8, ud: Direction) {
        // Set the up/down pin
        match ud {
            Direction::Up => self.ud.set_high(),
            Direction::Down => self.ud.set_low(),
        }
        // Turn the inc pin to high if not already set
        self.inc.set_high();
        // Enable the chip
        self.cs.set_low();
        delay_ms!(1);
        for _ in 1..=steps {
            // Trigger
            self.cs.set_high();
            delay_ms!(1);
            // Complete the toggle
            self.cs.set_high();
            delay_ms!(1);
        }
        // Disable the chip
        self.cs.set_high();
    }
    pub fn reset(&mut self, reset_direction: Direction) {
        self.wipe(self.steps, reset_direction);
        self.current_step = match reset_direction {
            Direction::Up => Some(self.steps),
            Direction::Down => Some(1),
        }
    }
    pub fn set(&mut self, step: u8) {
        let step = step.clamp(1, self.steps);
        if self.current_step == None {
            self.reset(Direction::Down);
        }
        let current_step = self.current_step.unwrap();
        let direction = match step.cmp(&current_step) {
            Ordering::Greater => Direction::Down,
            Ordering::Less => Direction::Up,
            Ordering::Equal => return,
        };
        let distance = (step as i16 + current_step as i16).abs() as u8;
        self.wipe(distance, direction);
        self.current_step = Some(step);
    }
}

impl Drop for Digipot {
    fn drop(&mut self) {
        self.cs.set_high();
    }
}
