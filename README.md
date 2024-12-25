<p align="center">
  <img alt="label-logger logo" src="https://raw.githubusercontent.com/mrnossiom/label-logger/main/assets/logo.png" />
</p>

<p align="center"><strong>
Cargo-like logging library
</strong></p>

<p align="center">
  <img alt="Nix Powered" src="https://img.shields.io/badge/Nix-Powered-blue?logo=nixos" />
  <a href="https://wakatime.com/badge/github/mrnossiom/label-logger">
    <img alt="Time spent" src="https://wakatime.com/badge/github/mrnossiom/label-logger.svg" />
  </a>
  <a href="https://crates.io/crates/label-logger">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/label-logger">
  </a>
</p>


# Usage

```rust
use label_logger::{info, log, success};

info!(label: "Compiling", "the program");
log!("information without label");
log!("more informations without label");
success!(label: "Finished", "the compilation");
```

The library also includes themes for [`dialoguer`](https://github.com/mitsuhiko/dialoguer/) (a library to prompt the user in the terminal) and [`indicatif`](https://github.com/console-rs/indicatif) (to show nice progress bars).

See [**examples**](https://github.com/mrnossiom/label-logger/tree/main/examples) for more use-case.

This library is still under `v1`, if necessary, breaking API changes can happen.

# Credits

-   **[woobuc/sweep](https://github.com/woobuc/sweep)** for the logging theme idea.
-   **[rust-lang/log](https://github.com/rust-lang/log)** for macros inspiration.

---

This work is licensed under [`CeCILL-2.1`](https://choosealicense.com/licenses/cecill-2.1), a strong copyleft French OSS license. This license allows modification and distribution of the software while requiring the same license for derived works.
