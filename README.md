![Label Logger](https://raw.githubusercontent.com/MrNossiom/git-leave/main/assets/logo.png)

# Label Logger

"A Cargo-like logging library."

## Installation

> This library is still under development and breaking API changes can happen at all time.

Add this to your `Cargo.toml`

```toml
label-logger = { git = "https://github.com/MrNossiom/label-logger", branch = "main" }
```

## Usage

```rust
use label_logger::{info, log, success};

info!(label: "Compiling", "the program");
log!("information without label");
log!("more informations without label");
success!(label: "Finished", "the compilation");
```

The library also includes themes for [`dialoguer`](https://github.com/mitsuhiko/dialoguer/) (a library to prompt the user in the terminal) and [`indicatif`](https://github.com/console-rs/indicatif) (to show nice progress bars).

See [**examples**](https://github.com/MrNossiom/label-logger/tree/main/examples) for more use-case.

## Credits

-   **[woobuc/sweep](https://github.com/woobuc/sweep)** for the logging theme idea.
-   **[rust-lang/log](https://github.com/rust-lang/log)** for macros inspiration.
