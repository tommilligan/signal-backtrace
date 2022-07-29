# signal-backtrace

Print a backtrace even when a fatal signal is received.

## Usage

Install the global handler before your code crashes:

```rust
fn main() {
    signal_backtrace::install();

    // the rest of your code
}
```

## Limitations

Only available on nightly.
