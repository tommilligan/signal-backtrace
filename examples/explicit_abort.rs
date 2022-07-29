//! When run, will self-abort and print the backtrace.

fn main() {
    signal_backtrace::install();

    std::process::abort();
}
