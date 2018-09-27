#![no_std]

extern crate tock;

use tock::buttons;
use tock::buttons::ButtonState;
use tock::console::Console;
use tock::timer;
use tock::timer::Duration;

fn main() {
    let mut console = Console::new();
    let mut with_callback = buttons::with_callback(|_, _| {});
    let mut buttons = with_callback.init().unwrap();
    let mut button = buttons.iter_mut().next().unwrap();
    let button = button.enable().unwrap();

    loop {
        match button.read() {
            ButtonState::Pressed => console.write("pressed\n"),
            ButtonState::Released => console.write("released\n"),
        }
        timer::sleep(Duration::from_ms(500));
    }
}
