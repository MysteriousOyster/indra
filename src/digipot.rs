extern crate rppal;
use std::{cmp::Ordering, thread, time::Duration};

use rppal::gpio::OutputPin;

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
    pub current_step: Option<u8>,
}

impl Digipot {
    /// Creates a new digital potentiometer struct from the given output pins and number of steps. The Digipot will not know where it is until the [`reset`](crate::Digipot::reset) method is called and the [`current_step`](crate::Digipot::current_step) field is updated.
    /// # Params
    /// - `cs`: an RPPAL [`OutputPin`](rppal::gpio::OutputPin) connected to the CS/CE (Chip select/chip enable) pin on the digital potentiometer. This pin is responsible for enabling or disabling the chip.
    /// - `inc`: another RPPAL `OutputPin` connected to the INC pin on the digital potentiometer
    /// - `ud`: another RPPAL `OutputPin` connected to the U/D pin on the digital potentiometer
    /// - `steps`: the number of steps the digital potentiometer will have. Keep in mind that the Digipot struct counts steps from 1 to the specified step value inclusive.
    /// # Examples
    /// ```rust
    /// # extern crate rppal;
    /// # use rppal::gpio;
    /// # fn main() -> gpio::Result<()> {
    /// use rppal::gpio::{OutputPin, Gpio};
    /// const DIGIPOT_CS: u8 = 4;
    /// const DIGIPOT_INC: u8 = 5;
    /// const DIGIPOT_UD: u8 = 6;
    /// let gpio = Gpio::new()?;
    /// let mut my_digipot = Digipot::new(
    ///     gpio.get(DIGIPOT_CS)?.into_output(),
    ///     gpio.get(DIGIPOT_INC)?.into_output(),
    ///     gpio.get(DIGIPOT_UD)?.into_output(),
    ///    100,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(cs: OutputPin, inc: OutputPin, ud: OutputPin, steps: u8) -> Self {
        Digipot {
            cs,
            inc,
            ud,
            steps,
            current_step: None,
        }
    }
    /// Wipes the digital potentiometer a certain distance
    /// # Params
    /// - `steps`: The number of steps to wipe (up or down, depending on the direction specified). If set to `0`, this function returns and does nothing.
    /// - `ud`: The [`Direction`](crate::digipot::Direction) in which the digipot should be going
    /// - `volatile`: Whether the wipe should be stored to volatile memory (`true`) or non-volatile memory (`false`).
    pub fn wipe(&mut self, mut steps: u8, ud: Direction, volatile: bool) {
        if steps == 0 {
            return;
        }
        // Set the up/down pin
        match ud {
            Direction::Up => self.ud.set_high(),
            Direction::Down => self.ud.set_low(),
        }
        // Turn the inc pin to high if not already set
        self.inc.set_high();
        // Enable the chip
        self.cs.set_low();
        thread::sleep(Duration::from_nanos(200));
        if volatile {
            self.inc.set_low();
            thread::sleep(Duration::from_micros(5));
            steps -= 1;
        }
        for _ in 1..=steps {
            // Trigger
            self.inc.toggle();
            thread::sleep(Duration::from_micros(5));
            self.inc.toggle();
            thread::sleep(Duration::from_micros(5));
        }
        // Disable the chip
        self.cs.set_high();
        thread::sleep(Duration::from_micros(5));
        // Set high (in event of volatile memory)
        self.inc.set_high();
        // We need to store the toggles we just did.
        thread::sleep(Duration::from_millis(20));
    }
    /// Wipes the Digipot to one extreme to that the Digipot struct knows where the Digipot is.
    /// # Params
    /// - `reset_direction`: The Direction in which to wipe the digital potentiometer.
    pub fn reset(&mut self, reset_direction: Direction) {
        self.wipe(self.steps, reset_direction, false);
        self.current_step = match reset_direction {
            Direction::Up => Some(self.steps),
            Direction::Down => Some(1),
        };
    }
    pub fn set_specified_volatile(&mut self, step: u8, volatile: bool) {
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
        let distance = (step as i16 - current_step as i16).abs() as u8;
        self.wipe(distance, direction, volatile);
        self.current_step = Some(step);
    }
    /// Sets the Digipot to the specified position. This will wipe the Digipot to the specified position and store position data in the struct. Setting a Digipot with this function will store the data in non-volatile memory, and will go back to that position when power-cycled, assuming another non-volaitle wipe has not been produced.
    /// # Params
    /// - `step`: Which step to set the Digipot to. Keep in mind the Digipot has steps from 1 to the stored `step` field specified in the [`new`](crate::digipot::Digipot::new) function.
    #[inline]
    pub fn set(&mut self, step: u8) {
        self.set_specified_volatile(step, false);
    }
}

impl Drop for Digipot {
    fn drop(&mut self) {
        self.cs.set_high();
    }
}
