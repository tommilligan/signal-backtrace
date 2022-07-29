#![feature(core_intrinsics)]

use backtrace::Backtrace;
use std::thread;

extern "C" fn handler(sig: i32) {
    let thread = thread::current();
    let name = thread.name().unwrap_or("<unnamed>");
    println!(
        "Received signal '{}' in thread '{}'. Stack trace\n{:?}",
        sig,
        name,
        Backtrace::new()
    );

    // Basically `std::intrinsics::abort()` but able to run on stable.
    //
    // Execute an invalid assembly instruction.
    std::intrinsics::abort();
}

pub fn install() {
    use sig::{ffi::Sig, signal};

    // handle segfaults
    signal!(Sig::SEGV, handler);
    // handle stack overflow and unsupported CPUs
    signal!(Sig::ILL, handler);
    // handle double panics
    // note: alias for SIGABRT
    signal!(Sig::IOT, handler);
    // handle invalid memory access
    signal!(Sig::BUS, handler);
}
