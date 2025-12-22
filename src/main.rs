use rust_gpiozero::{LED, DigitalOutputDevice};
use std::{sync::{Arc, Mutex}, cmp::Ordering};

const INDICATOR_LED: u8 = 10;
const DIGIPOT_CS: u8 = 0;
const DIGIPOT_INC: u8 = 0;
const DIGIPOT_UD: u8 = 0;

macro_rules! delay_ms {
    ($e:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($e));
    };
}

fn main() {
    println!("INDRA program starting...");

    // Initialize indicator LED and turn it on
    let indicator_led: IndicatorLed = IndicatorLed::new(INDICATOR_LED);
    indicator_led.set(IndicatorStatus::Sleep);

    // Digipot! 
    let mut digipot = Digipot::new(DIGIPOT_CS, DIGIPOT_INC, DIGIPOT_UD, 100);
    digipot.cs = DigitalOutputDevice::new(1);
    
    delay_ms!(500);
}

/// Enumerator to indicate the status of the indicator LED
#[allow(dead_code)]
enum IndicatorStatus {
    Safe,
    Unsafe,
    Sleep,
}

/// Struct to interface with the indicator LED
#[derive(Debug)]
struct IndicatorLed(Arc<Mutex<LED>>);

impl IndicatorLed {
    fn new(pin: u8) -> Self {
        IndicatorLed(Arc::new(Mutex::new(
            LED::new(pin)
        )))
    }

    /// Set an indicator led
    /// # Panics
    /// This will panic if the contained LED object is posoined.
    fn set(&self, state: IndicatorStatus) {
        let mut led= self.0.lock().unwrap();
        match state {
            IndicatorStatus::Safe => led.blink(1.0, 2.0),
            IndicatorStatus::Unsafe => led.blink(0.5, 0.5),
            IndicatorStatus::Sleep => led.on(),
        }
    }
}

impl Clone for IndicatorLed {
    fn clone(&self) -> Self {
        IndicatorLed(Arc::clone(&self.0))
    }
}

/// Enum for digipot up/down
#[allow(dead_code)]
enum DigipotDirection {
    Up,
    Down,
}

/*
impl Into<bool> for DigipotDirection {
    fn into(self) -> bool {
        match self {
            Self::Up => true,
            Self::Down => false,
        }
    }
}

impl Into<i8> for DigipotDirection {
    fn into(self) -> i8 {
        match self {
            Self::Up => 1,
            Self::Down => -1
        }
    }
}
*/

/// Digital potentiometer
#[derive(Debug)]
struct Digipot {
    /// Chip select device. Ground to enable.
    cs: DigitalOutputDevice,
    /// Increment (basically we pulse this to change the value)
    inc: DigitalOutputDevice,
    /// Up/down - up is high, down is low.
    ud: DigitalOutputDevice,
    /// Bounds in steps of the chip.
    steps: u8,
    current_step: Option<u8>,
}

impl Digipot {
    /// Construct a new Digipot.
    /// # Parameters
    /// - `cs_pin`: chip select
    /// - `inc_pin`: increment pin
    /// - `ud_pin`: Up/Down pin, usually represented as U/D
    /// - `steps`: The steps of the potentiometer. Should be
    ///     at least one, so steps will clamp to 1 if it is 0.
    fn new(cs_pin: u8, inc_pin: u8, ud_pin: u8, steps: u8) -> Self {
        let mut digipot = Self {
            cs: DigitalOutputDevice::new(cs_pin),
            inc: DigitalOutputDevice::new(inc_pin),
            ud: DigitalOutputDevice::new(ud_pin),
            steps: steps.clamp(1, u8::MAX),
            current_step: None,
        };

        // Increment to highest value at the start
        digipot.set(steps);

        digipot
    }

    /// Increment the digipot by a specified number of steps.
    /// Not recommended for use, as it does not update the structure's
    /// internal step counter.
    /// # Parameters
    /// - `direction`: Direction to increment by
    /// - `ticks`: How many ticks it should increment in a specific direction
    fn increment(&self, direction: DigipotDirection, ticks: u8) {
        // Enable the potentiometer.
        self.cs.off();
        match direction {
            DigipotDirection::Up => self.ud.on(),
            DigipotDirection::Down => self.ud.off(),
        }
        for _ in 0..ticks {
            delay_ms!(1);
            self.inc.off();
            delay_ms!(1);
            self.inc.on();
        }
        delay_ms!(1);
        self.cs.off();
    }

    /// Sets the potentiometer's value in steps.
    /// # Parameters
    /// - `step`: Step, in u8, that this should increment towards.
    ///     This value will be clamped to 1 and the maximum number of steps.
    fn set(&mut self, step: u8) {
        let current_step = {
            if let Some(current_step) = self.current_step {
                current_step
            } else {
                self.increment(DigipotDirection::Up, self.steps);
                self.current_step = Some(self.steps);
                self.steps
            }
        };
        let target_step = step.clamp(1, self.steps);
        let direction = match current_step.cmp(&target_step) {
            Ordering::Equal => return,
            Ordering::Less => DigipotDirection::Down,
            Ordering::Greater => DigipotDirection::Up,
        };
        let difference = current_step.abs_diff(target_step);
        self.increment(direction, difference);
        self.current_step = Some(target_step);
    }
}