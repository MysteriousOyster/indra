extern crate rppal;
extern crate lerp;

use std::{ops::{Deref, DerefMut}, time::Duration};

use lerp::Lerp;
use rppal::pwm::{self, Channel, Pwm};

pub struct Servo {
    min_duty: f64,
    max_duty: f64,
    pwm: Pwm,
}

impl Servo {
    pub fn new(channel: Channel) -> pwm::Result<Self> {
        Self::new_with_min_max(channel, 0.05, 0.1)
    }

    pub fn new_with_min_max(channel: Channel, min_duty: f64, max_duty: f64) -> pwm::Result<Self> {
        let pwm = Pwm::new(channel)?;
        pwm.set_period(Duration::from_millis(20))?;
        pwm.enable()?;
        Ok(Self {
            min_duty,
            max_duty,
            pwm,
        })
    }
    
    pub fn set_percent(&self, percent: f64) -> pwm::Result<()> {
        let duty_cycle = self.min_duty.lerp(self.max_duty, percent / 100.0);
        self.set_duty_cycle(duty_cycle)
    }
}

impl Deref for Servo {
    type Target = Pwm;
    fn deref(&self) -> &Self::Target {
        &self.pwm
    }
}

impl DerefMut for Servo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pwm
    }
}