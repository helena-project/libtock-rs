use callback::CallbackSubscription;
use callback::SubscribableCallback;
use result::TockError;
use result::TockResult;
use shared_memory::SharedMemory;

pub fn yieldk() {
    // Note: A process stops yielding when there is a callback ready to run,
    // which the kernel executes by modifying the stack frame pushed by the
    // hardware. The kernel copies the PC value from the stack frame to the LR
    // field, and sets the PC value to callback to run. When this frame is
    // unstacked during the interrupt return, the effectively clobbers the LR
    // register.
    //
    // At this point, the callback function is now executing, which may itself
    // clobber any of the other caller-saved registers. Thus we mark this
    // inline assembly as conservatively clobbering all caller-saved registers,
    // forcing yield to save any live registers.
    //
    // Upon direct observation of this function, the LR is the only register
    // that is live across the SVC invocation, however, if the yield call is
    // inlined, it is possible that the LR won't be live at all (commonly seen
    // for the `loop { yieldk(); }` idiom) or that other registers are live,
    // thus it is important to let the compiler do the work here.
    //
    // According to the AAPCS: A subroutine must preserve the contents of the
    // registers r4-r8, r10, r11 and SP (and r9 in PCS variants that designate
    // r9 as v6) As our compilation flags mark r9 as the PIC base register, it
    // does not need to be saved. Thus we must clobber r0-3, r12, and LR
    unsafe {
        asm!(
            "svc 0"
            :
            :
            : "memory", "r0", "r1", "r2", "r3", "r12", "lr"
            : "volatile");
    }
}

pub fn yieldk_for<F: Fn() -> bool>(cond: F) {
    while !cond() {
        yieldk();
    }
}

pub fn subscribe<CB: SubscribableCallback>(
    driver_number: usize,
    subscribe_number: usize,
    callback: &mut CB,
) -> TockResult<CallbackSubscription> {
    let return_code = unsafe {
        subscribe_ptr(
            driver_number,
            subscribe_number,
            c_callback::<CB> as *const _,
            callback as *mut CB as usize,
        )
    };

    TockError::from_return_code(return_code)
        .map(|_| CallbackSubscription::new(driver_number, subscribe_number))
}

extern "C" fn c_callback<CB: SubscribableCallback>(
    arg0: usize,
    arg1: usize,
    arg2: usize,
    userdata: usize,
) {
    let callback = unsafe { &mut *(userdata as *mut CB) };
    callback.call_rust(arg0, arg1, arg2);
}

pub unsafe fn subscribe_ptr(
    major: usize,
    minor: usize,
    cb: *const unsafe extern "C" fn(usize, usize, usize, usize),
    ud: usize,
) -> isize {
    let return_code;
    asm!("svc 1" : "={r0}"(return_code)
                 : "{r0}"(major) "{r1}"(minor) "{r2}"(cb) "{r3}"(ud)
                 : "memory"
                 : "volatile");
    return_code
}

pub unsafe fn command(major: usize, minor: usize, arg1: usize, arg2: usize) -> TockResult<usize> {
    let return_code;
    asm!("svc 2" : "={r0}"(return_code)
                 : "{r0}"(major) "{r1}"(minor) "{r2}"(arg1) "{r3}"(arg2)
                 : "memory"
                 : "volatile");
    TockError::from_return_code(return_code)
}

pub fn allow(
    driver_number: usize,
    allow_number: usize,
    buffer_to_share: &mut [u8],
) -> TockResult<SharedMemory> {
    let len = buffer_to_share.len();
    let return_code = unsafe {
        allow_ptr(
            driver_number,
            allow_number,
            buffer_to_share.as_mut_ptr(),
            len,
        )
    };

    TockError::from_return_code(return_code).map(move |_| SharedMemory {
        driver_number,
        allow_number,
        buffer_to_share,
    })
}

pub unsafe fn allow_ptr(major: usize, minor: usize, slice: *mut u8, len: usize) -> isize {
    let return_code;
    asm!("svc 3" : "={r0}"(return_code)
                 : "{r0}"(major) "{r1}"(minor) "{r2}"(slice as *mut u8) "{r3}"(len)
                 : "memory"
                 : "volatile");
    return_code
}

pub unsafe fn memop(major: u32, arg1: usize) -> isize {
    let res;
    asm!("svc 4" : "={r0}"(res)
                 : "{r0}"(major) "{r1}"(arg1)
                 : "memory"
                 : "volatile");
    res
}
