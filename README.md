# Label Logger

> A Cargo-like logging library.

# Usage

```
#[macro_use]
extern crate label_logger;

use label_logger::OutputLabel;

fn main() {
    // Log what you want.
    success!("Hello", "world");
    info!("Info", "world");
    eprintln!("Bye, {}!", "program");
}
```

> See [examples](https://github.com/MrNossiom/label_logger/tree/main/examples) for more use-case

## Thanks

-   **[woobuc/sweep](https://github.com/woobuc/sweep)** for the logging theme idea.
