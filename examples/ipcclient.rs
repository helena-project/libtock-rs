#![feature(alloc)]
#![no_std]

extern crate alloc;
extern crate tock;

use alloc::string::String;
use core::fmt::Write;
use tock::console::Console;
use tock::ipc;
use tock::ipc::IpcClientCallback;
use tock::ipc::ServerHandle;
use tock::timer;

// Calls the ipc_server and prints result
fn main() {
    let mut server_buf = ipc::reserve_shared_buffer();
    let mut my_buf = ipc::reserve_shared_buffer();
    timer::sleep(timer::Duration::from_ms(1000));

    loop {
        let mut server = ServerHandle::discover_service(String::from("ipcserver")).unwrap();
        let payload: [u8; 32] = [5; 32];

        let mut handle = server.share(&mut server_buf).unwrap();
        handle.write_bytes(&payload);

        let mut callback = IpcClientCallback::new(|_: usize, _: usize| {
            handle.read_bytes(&mut my_buf.buffer);
            let mut console = Console::new();
            writeln!(console, "Client: \"Payload: {}\"", my_buf.buffer[0]);
        });

        let handle = server.subscribe_callback(&mut callback);

        server.notify().unwrap();
        timer::sleep(timer::Duration::from_ms(1000));
        handle.unwrap();
    }
}
