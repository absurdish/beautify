# Beautify

A simple crate for beautifying your terminal by coloring it

```sh
# installation
cargo add beautify
```

```rs
// usage
use beautify::Colors;

fn main() {
    println!("{}", "accent color".text_blue_600());
    println!("{}", "black text, white background".text_black().bg_white());
    println!("{}", "my gradient".text_gradient(&["#ffffff", "#000000"]));
}
```

more features will be added in the near future.
