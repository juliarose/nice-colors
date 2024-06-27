# nice-colors

Provides a wrapper that represents a color with RGB color values along with methods commonly 
used for manipulating, formatting, and parsing colors.

## Usage

```rust
use nice_colors::Color;

let red = Color { red: 255, green: 0, blue: 0 };
let blue = Color { red: 0, green: 0, blue: 255 };
let amount = 0.5;
let blended = red.blend(blue, amount);

assert_eq!(
    blended,
    Color { red: 128, green: 0, blue: 128 },
);
assert_eq!(blended.to_hex_string(), "#800080");
assert_eq!(blended.to_rgb_string(), "rgb(128,0,128)");
assert_eq!(blended.to_rgba_string(0.5), "rgba(128,0,128,0.5)");
assert_eq!(
    Color::from(0xFF0000),
    Color{ red: 255, green: 0, blue: 0 },
);
assert_eq!(
    Color::from_hex_str("#800080").unwrap(),
    Color { red: 128, green: 0, blue: 128 },
);
assert_eq!(
    Color::from_hex_str("#F00").unwrap(),
    Color { red: 255, green: 0, blue: 0 },
);
```

## License

[MIT](https://github.com/juliarose/nice-colors/blob/master/LICENSE)