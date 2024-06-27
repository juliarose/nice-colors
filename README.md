# nice-colors

Provides a wrapper that represents a color with RGB color values along with methods commonly 
used for manipulating, formatting, and parsing colors.

## Usage

```rs
use nice_colors::Color;

let red = Color { r: 255, g: 0, b: 0 };
let blue = Color { r: 0, g: 0, b: 255 };
let amount = 0.5;
let blended = red.blend(blue, amount);

assert_eq!(blended, Color { r: 128, g: 0, b: 128 });
assert_eq!(blended.to_hex(), "800080");
assert_eq!(blended.to_rgb(), "rgb(128,0,128)");
assert_eq!(Color::from_decimal(0xFF0000).unwrap(), Color{ r: 255, g: 0, b: 0 });
assert_eq!(Color::from_hex("800080").unwrap(), Color { r: 128, g: 0, b: 128 });
assert_eq!(Color::from_hex("#800080").unwrap(), Color { r: 128, g: 0, b: 128 });
assert_eq!(Color::from_hex("F00").unwrap(), Color { r: 255, g: 0, b: 0 });
```

## License

[MIT](https://github.com/juliarose/nice-colors/blob/master/LICENSE)