# Label Logger

> A Cargo-like logging library.

# Installation

> This library is still under development and breaking API changes can happen at all time.

Add this to your `Cargo.toml`

```toml
label-logger = { git = "https://gituhb.com/MrNossiom/label-logger", branch = "main" }
```

# Usage

```rust
// You can either import all the macros at once globally...
#[macro_use] extern crate label_logger;
// ...or import them one by one in every crate
use label_logger::{info, log, success};

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
