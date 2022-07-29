#![feature(core_intrinsics)]

extern "C" fn handler(sig: i32) {
    let thread = std::thread::current();
    let name = thread.name().unwrap_or("<unnamed>");
    println!(
        "Received signal '{}' in thread '{}'. Backtrace:\n{:?}",
        sig,
        name,
        backtrace::Backtrace::new()
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
