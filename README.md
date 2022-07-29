# signal-backtrace

[![Crates.io](https://img.shields.io/crates/v/signal-backtrace)](https://crates.io/crates/signal-backtrace)
[![GitHub](https://img.shields.io/github/license/tommilligan/signal-backtrace)](https://github.com/tommilligan/signal-backtrace/blob/master/LICENSE)

Print a backtrace even when a fatal signal is received.

## Usage

Install the global handler before your code crashes:

```rust
fn main() {
    signal_backtrace::install();

    // the rest of your code
}
```

## Output

Run `cargo +nightly run --example explicit_abort` for a working example.

The backtrace will include the handling code itself, followed by the cause;
in this case, an explicit call to `std::process::abort`:

```log
Received signal '6' in thread 'main'. Backtrace:
   0: signal_backtrace::handler
             at src/lib.rs:10:9
   1: <unknown>
   2: __libc_signal_restore_set
             at /build/glibc-SzIz7B/glibc-2.31/signal/../sysdeps/unix/sysv/linux/internal-signals.h:86:3
      __GI_raise
             at /build/glibc-SzIz7B/glibc-2.31/signal/../sysdeps/unix/sysv/linux/raise.c:48:3
   3: __GI_abort
             at /build/glibc-SzIz7B/glibc-2.31/stdlib/abort.c:79:7
   4: std::sys::unix::abort_internal
             at /rustc/10f4ce324baf7cfb7ce2b2096662b82b79204944/library/std/src/sys/unix/mod.rs:292:14
   5: std::process::abort
             at /rustc/10f4ce324baf7cfb7ce2b2096662b82b79204944/library/std/src/process.rs:2103:5
   6: explicit_abort::main
             at examples/explicit_abort.rs:4:5
...
```

## Limitations

Only available on nightly.
