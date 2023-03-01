# nice-colors

For converting colors to and from hexadecimal and blending colors.

## Usage

```rs
use nice_colors::Color;

let red = Color::new(255, 0, 0);
let blue = Color::new(0, 0, 255);
let amount = 0.5;
let blended = red.blend(blue, amount);

assert_eq!(blended, Color::new(128, 0, 128));
assert_eq!(blended.to_hex(), "800080");
assert_eq!(blended.to_rgb(), "rgb(128,0,128)");
assert_eq!(Color::from_hex("800080").unwrap(), Color::new(128, 0, 128));
assert_eq!(Color::from_hex("F00").unwrap(), Color::new(255, 0, 0));
```

## License

[MIT](https://github.com/juliarose/nice-colors/blob/master/LICENSE)