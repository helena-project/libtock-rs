#![no_std]

extern crate tock;

use core::fmt::Write;
use tock::console::Console;
use tock::temperature;

fn main() {
    let mut console = Console::new();

    let mut with_callback = temperature::with_callback(|result: isize| {
        writeln!(console, "Temperature: {}", result);
    });

    let _temperature = with_callback.start_measurement();

    loop {
        tock::syscalls::yieldk();
    }
}
