# beautify 🎨

A simple Rust crate for beautifying your terminal output with colors, gradients, and text effects with more then 300 methods.

## Installation

```sh
cargo add beautify
```

## Features

### Colors

```rust
use beautify::Colors;

// Basic colors
println!("{}", "Hello".text_blue());
println!("{}", "World".bg_red());

// Accent colors (with shades 50-950)
println!("{}", "Modern".text_blue_600());
println!("{}", "Design".bg_purple_200());

// Multiple styles
println!("{}", "Stylish Text".text_pink_400().bg_black());
```

### Gradients

```rust
use beautify::Colors;

// Text gradients
println!("{}", "Rainbow Text".text_gradient(&["#ff0000", "#00ff00", "#0000ff"]));

// Background gradients
println!("{}", "Sunset".bg_gradient(&["#ff7e5f", "#feb47b"]));
```

### Text Effects

```rust
use beautify::Colors;

// Basic formatting
println!("{}", "Bold Text".bold());
println!("{}", "Italic Text".italic());
println!("{}", "Underlined".underline());

// Animations
println!("{}", "Blinking".blink());
println!("{}", "Fast Blink".blink_fast());
println!("{}", "Slow Blink".blink_slow());

// Fading effects
println!("{}", "Fade In".fade_in());
println!("{}", "Fade Out".fade_out());
```

### Text Layout

```rust
use beautify::Colors;

// Padding and alignment
println!("{}", "Centered".center(20));
println!("{}", "Right Aligned".pad_left(20));
println!("{}", "Left Aligned".pad_right(20));

// Borders
println!("{}", "Boxed Text".box_it());
println!("{}", "Single Border".border());
println!("{}", "Double Border".double_border());
```

## Color Palettes

- Default colors: black, red, green, blue, yellow, magenta, cyan, white
- Bright variants: bright_red, bright_blue, etc.
- Modern accent colors with 11 shades (50-950):
  - Red: `text_red_50()` to `text_red_950()`
  - Blue: `text_blue_50()` to `text_blue_950()`
  - Green: `text_green_50()` to `text_green_950()`
  - Yellow: `text_yellow_50()` to `text_yellow_950()`
  - Purple: `text_purple_50()` to `text_purple_950()`
  - Pink: `text_pink_50()` to `text_pink_950()`
  - Black/Gray: `text_black_50()` to `text_black_950()`

All colors are available as both text colors (`text_*`) and background colors (`bg_*`).

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.

## License

This project is licensed under the MIT License.
