# parse-color

A crate that provides conversion from CSS color names or TailwindCSS classes to RGBA colors, in the form of [u8; 4].

## Examples

```rust
assert_eq!(parse_color::parse("Red"), Some([255, 0, 0, 255]));
assert_eq!(parse_color::parse("Transparent"), Some([0, 0, 0, 0]));
assert_eq!(parse_color::parse("light coral"), Some([240, 128, 128, 255]));
assert_eq!(parse_color::parse("Rebecca-Purple"), Some([102, 51, 153, 255]));

// note the 0 value is only allowed on black/white/transparent
assert_eq!(parse_color::parse_tailwind("white", 0), Some([255, 255, 255, 255]));
assert_eq!(parse_color::parse_tailwind("sky", 400), Some([56, 189, 248, 255]));
assert_eq!(parse_color::parse_tailwind("fuchsia", 900), Some([112, 26, 117, 255]));
```
