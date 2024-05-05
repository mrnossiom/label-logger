![Label Logger](https://raw.githubusercontent.com/MrNossiom/label-logger/main/assets/logo.png)

<p align="center"><strong>
Cargo-like logging library
</strong></p>

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

This library is still under development and breaking API changes can happen at all time.

---

Work is licensed under [`CECILL-2.1`](https://choosealicense.com/licenses/cecill-2.1/), a French OSS license that allows modification and distribution of the software while requiring the same license for derived works.
