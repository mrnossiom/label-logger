# Label Logger

> A Cargo-like logging library.

# Usage

```rust
#[macro_use] extern crate label_logger;

fn main() {
    info!(label: "Compiling", "the program");
    log!("information without label");
    log!("more informations without label");
    success!(label: "Finished", "the compilation");
}
```

> See [examples](https://github.com/MrNossiom/label-logger/tree/main/examples) for more use-case

## Thanks

-   **[woobuc/sweep](https://github.com/woobuc/sweep)** for the logging theme idea.
-   **[rust-lang](https://github.com/rust-lang/log)** for macros inspiration.
