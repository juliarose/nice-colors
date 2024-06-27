use crate::color::{SLICE_LENGTH, Alpha, Value};
use crate::helpers::{self, conversions};

/// Attempts to parse a hexadecimal color string into a color.
pub fn hex(mut hex: &str, must_include_hash: bool) -> Option<[u8; SLICE_LENGTH]> {
    let mut len = hex.len();
    
    if hex.starts_with('#') {
        hex = &hex[1..len];
        len -= 1;
    } else if must_include_hash {
        return None;
    }
    
    if !matches!(len, 3 | 4 | 6 | 8) {
        return None;
    }
    
    let decimal = u32::from_str_radix(hex, 16).ok()?;
    
    return match len {
        3 => Some([
            (((decimal >> 8) & 0xF) * 0x11) as Value, // Red
            (((decimal >> 4) & 0xF) * 0x11) as Value, // Green
            ((decimal & 0xF) * 0x11) as Value, // Blue
        ]),
        4 => Some([
            (((decimal >> 12) & 0xF) * 0x11) as Value, // Red
            (((decimal >> 8) & 0xF) * 0x11) as Value, // Green
            (((decimal >> 4) & 0xF) * 0x11) as Value, // Blue
            // Skip alpha
        ]),
        6 => Some([
            ((decimal >> 16) & 0xFF) as Value, // Red
            ((decimal >> 8) & 0xFF) as Value, // Green
            (decimal & 0xFF) as Value, // Blue
        ]),
        8 => Some([
            ((decimal >> 24) & 0xFF) as Value, // Red
            ((decimal >> 16) & 0xFF) as Value, // Green
            ((decimal >> 8) & 0xFF) as Value, // Blue
            // Skip alpha
        ]),
        // Never actually reached with the "matches" check above
        _ => None,
    };
}

/// Converts an HSL color string to a slice of R, G, B color values as u8 integers.
pub fn hsl(mut hsl: &str) -> Option<([u8; SLICE_LENGTH], Alpha)> {
    let mut len = hsl.len();
    let mut colors_expected = 3;
    
    if hsl.starts_with("hsl(") {
        hsl = &hsl[4..len];
        len -= 4;
    } else if hsl.starts_with("hsla(") {
        hsl = &hsl[5..len];
        len -= 5;
        colors_expected += 1;
    } else {
        return None;
    }
    
    if hsl.ends_with(')') {
        hsl = &hsl[..(len - 1)];
    } else {
        return None;
    }
    
    let mut hue = None;
    let mut saturation = None;
    let mut lightness = None;
    let mut alpha = 1.0;
    let mut i = 0;
    
    for mut c in hsl.split([',', ' ']) {
        c = c.trim();
        
        // Skip empty strings
        if c.is_empty() {
            continue;
        }
        
        match i {
            0 => {
                let parsed = c.parse::<u16>().ok()?;
                let value = (((parsed as f32 % 360.0) + 360.0) % 360.0) / 360.0;
                
                hue = Some(value);
            },
            1 => {
                saturation = helpers::parse_percent(c);
            },
            2 => {
                lightness = helpers::parse_percent(c);
            },
            3 if colors_expected == 4 => {
                alpha = helpers::parse_percent(c)?;
            },
            // Too many colors - invalid color
            _ => return None,
        }
        
        i += 1;
    }
    
    // Check if the number of colors is valid.
    if i != colors_expected {
        return None;
    }
    
    let hue = hue?;
    let saturation = saturation?;
    let lightness = lightness?;
    let m2 = if lightness <= 0.5 {
        lightness * (saturation + 1.0)
    } else {
        lightness + saturation - lightness * saturation
    };
    let m1 = lightness * 2.0 - m2;
    let r = helpers::float_to_value(conversions::hue_to_rgb(m1, m2, hue + 1.0 / 3.0) * 255.0);
    let g = helpers::float_to_value(conversions::hue_to_rgb(m1, m2, hue) * 255.0);
    let b = helpers::float_to_value(conversions::hue_to_rgb(m1, m2, hue - 1.0 / 3.0) * 255.0);
    
    Some(([r, g, b], alpha))
}

/// Attempts to parse an rgb or rgba color string into a color. Alpha value defaults to `1.0` if 
/// not present.
pub fn rgba(mut rgb: &str) -> Option<([u8; SLICE_LENGTH], Alpha)> {
    let mut len = rgb.len();
    let mut colors_expected = SLICE_LENGTH;
    
    if rgb.starts_with("rgb(") {
        rgb = &rgb[4..len];
        len -= 4;
    } else if rgb.starts_with("rgba(") {
        rgb = &rgb[5..len];
        len -= 5;
        colors_expected += 1;
    } else {
        return None;
    }
    
    if rgb.ends_with(')') {
        rgb = &rgb[..(len - 1)];
    } else {
        return None;
    }
    
    let mut colors = [0u8; 3];
    let mut alpha: Alpha = 1.0;
    let mut i = 0;
    
    for mut c in rgb.split([',', ' ']) {
        c = c.trim();
        
        // Skip empty strings
        if c.is_empty() {
            continue;
        }
        
        match i {
            0..=2 => if c.ends_with('%') {
                // It's a percentage.
                colors[i] = (helpers::parse_percent(c)? * 255.0).round() as u8;
            } else if c.starts_with('-') {
                // Remove the negative sign.
                c = &c[1..];
                // See if it's a number.
                c.parse::<u32>().ok()?;
                // Negative numbers 
                colors[i] = 0;
            } else {
                // Numbers over 255 are acceptable.
                // Casting to u8 will truncate the value.
                colors[i] = u32::from_str_radix(c.trim(), 10).ok()? as u8;
            },
            3 if colors_expected == 4 => if let Ok(value) = u8::from_str_radix(c.trim(), 10) {
                alpha = value as f32 / Value::MAX as Alpha;
            } else {
                alpha = c.trim().parse::<Alpha>().ok()?;
            }
            // Too many colors - invalid color
            _ => return None,
        }
        
        i += 1;
    }
    
    // Check if the number of colors is valid.
    if i != colors_expected {
        return None;
    }
    
    Some((colors, alpha))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn remove_suffix_test() {
        assert_eq!(helpers::remove_suffix("hello", "world"), None);
        assert_eq!(helpers::remove_suffix("hello", "lo"), Some("hel"));
        assert_eq!(helpers::remove_suffix("hello", "llo"), Some("he"));
        assert_eq!(helpers::remove_suffix("hello", "hello"), Some(""));
        assert_eq!(helpers::remove_suffix("100%", "%"), Some("100"));
    }
    
    #[test]
    fn parses_hsl() {
        assert_eq!(hsl("hsl(0, 100%, 50%)"), Some(([255, 0, 0], 1.0)));
        assert_eq!(hsl("hsl(120, 100%, 50%)"), Some(([0, 255, 0], 1.0)));
        assert_eq!(hsl("hsl(0 100% 50%)"), Some(([255, 0, 0], 1.0)));
    }
}