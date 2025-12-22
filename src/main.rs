use rppal::gpio;

// TODO - Change these pins
const INDICATOR_PIN: u8 = 1;

fn main() {
    println!("Hello, world.");
    let gpio_interface = gpio::Gpio::new().unwrap();
    let mut indicator = gpio_interface.get(INDICATOR_PIN).unwrap().into_output();
    indicator.set_low();
}