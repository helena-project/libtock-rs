use libtock_platform::CommandReturn;

/// The `fake::Driver` trait is implemented by fake versions of Tock's kernel
/// APIs. It is used by `fake::Kernel` to route system calls to the fake kernel
/// APIs.
pub trait Driver: 'static {
    /// Returns this driver's ID. Used by `fake::Kernel` to route syscalls to
    /// the correct `fake::Driver` instance.
    fn id(&self) -> u32;

    // -------------------------------------------------------------------------
    // Subscribe
    // -------------------------------------------------------------------------

    // TODO: Add a Subscribe API.

    // -------------------------------------------------------------------------
    // Command
    // -------------------------------------------------------------------------

    /// Process a Command system call. Fake drivers should use the methods in
    /// `libtock_unittest::command_return` to construct the return value.
    fn command(&self, command_id: u32, argument0: u32, argument1: u32) -> CommandReturn;

    // -------------------------------------------------------------------------
    // Allow
    // -------------------------------------------------------------------------

    // TODO: Add an Allow API.
}
