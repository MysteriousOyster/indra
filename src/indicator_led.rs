extern crate rppal;

use rppal::gpio::OutputPin;
use std::{
    sync::{
        Arc, Mutex, MutexGuard, PoisonError,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Instant,
};

use super::future_sec_f64;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SafetyState {
    Sleep,
    Safe,
    Unsafe,
}

impl Into<(f64, f64)> for SafetyState {
    fn into(self) -> (f64, f64) {
        match self {
            Self::Sleep => (f64::MAX, 0.0),
            Self::Safe => (1.0, 2.0),
            Self::Unsafe => (0.5, 0.5),
        }
    }
}

impl Default for SafetyState {
    fn default() -> Self {
        Self::Sleep
    }
}

pub struct IndicatorLED {
    output: Arc<Mutex<OutputPin>>,
    handler_thread: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>,
    state: Arc<Mutex<SafetyState>>,
}

impl IndicatorLED {
    pub fn new(mut output: OutputPin) -> Self {
        output.set_high();
        let output = Arc::new(Mutex::new(output));
        let t_output = output.clone();
        let stop_flag = Arc::new(AtomicBool::default());
        let t_stop_flag = stop_flag.clone();
        let state = Arc::new(Mutex::new(SafetyState::default()));
        let t_state = state.clone();
        Self {
            output,
            handler_thread: Some(thread::spawn(move || {
                Self::handler_function(t_output, t_stop_flag, t_state)
            })),
            stop_flag,
            state,
        }
    }

    pub fn set(&mut self, state: SafetyState) -> Result<(), PoisonError<MutexGuard<SafetyState>>> {
        let mut current_state = self.state.lock()?;
        *current_state = state;
        Ok(())
    }

    /// The handler function that the indicator LED runs.
    /// # Panics
    /// Panics if the `state` Mutex is poisoned.
    fn handler_function(
        output: Arc<Mutex<OutputPin>>,
        stop_flag: Arc<AtomicBool>,
        state: Arc<Mutex<SafetyState>>,
    ) {
        let mut last_state = state.lock().unwrap();
        let (mut on_time, mut off_time): (f64, f64) = (*last_state).into();
        let mut next_trigger = future_sec_f64!(on_time);
        while !stop_flag.load(Ordering::Acquire) {
            if let Ok(state) = state.try_lock() {
                if *last_state != *state {
                    *last_state = *state;
                    (on_time, off_time) = (*state).into();
                }
            }
            if Instant::now() > next_trigger {
                if let Ok(mut output) = output.try_lock() {
                    if output.is_set_high() {
                        output.set_low();
                        next_trigger = future_sec_f64!(off_time);
                    } else {
                        output.set_high();
                        next_trigger = future_sec_f64!(on_time);
                    }
                }
            }
        }
    }
}

impl Drop for IndicatorLED {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Release);
        if let Some(handle) = self.handler_thread.take() {
            handle.join();
        }
    }
}
