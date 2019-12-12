#![no_std]

use core::fmt::Write;
use libtock::console::Console;
use libtock::result::TockResult;
use libtock::timer;
use libtock::timer::Duration;

libtock::async_main!(async_main);
fn async_main() -> TockResult<()> {
    let mut console = Console::new();

    for i in 0.. {
        writeln!(console, "Hello world! {}", i)?;
        timer::sleep(Duration::from_ms(500)).await?;
    }

    Ok(())
}
