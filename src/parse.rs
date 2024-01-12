use crate::{SLICE_LENGTH, Alpha, Value};

mod helpers {
    use crate::Value;
    
    /// Removes a suffix from a string if it exists. Returns `None` if the suffix does not exist.
    pub fn remove_suffix<'a>(
        s: &'a str,
        suffix: &str,
    ) -> Option<&'a str> {
        if s.ends_with(suffix) {
            let end = s.len() - suffix.len();
            
            return Some(&s[..end]);
        }
        
        None
    }
    
    /// Fits a percentage into the range of 0.0 to 1.0.
    pub fn fit_percent(value: f32) -> f32 {
        if value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        }
    }
    
    /// Parses a percentage value from a string.
    pub fn parse_percent(s: &str) -> Option<f32> {
        let value = remove_suffix(s.trim(), "%")?.parse::<f32>().ok()?;
        let percent = fit_percent(value / 100.0);
        
        Some(percent)
    }
    
    /// Converts a floating point value to a u8 integer.
    pub fn float_to_value(mut value: f32) -> Value {
        value = value.round();
        
        if value < 0.0 {
            0
        } else if value > 255.0 {
            255
        } else {
            value as Value
        }
    }
    
    /// Converts a hue to RGB.
    pub fn hue_to_rgb(m1: f32, m2: f32, mut h: f32) -> f32 {
        // Sourced from: https://github.com/7thSigil/css-color-parser-rs/blob/v0.1.2/src/color/color.rs#L366
        if h < 0.0 {
            h += 1.0;
        } else if h > 1.0 {
            h -= 1.0;
        }
        
        if h * 6.0 < 1.0 {
            return m1 + (m2 - m1) * h * 6.0;
        }
        
        if h * 2.0 < 1.0 {
            return m2;
        }
        
        if h * 3.0 < 2.0 {
            return m1 + (m2 - m1) * (2.0 / 3.0 - h) * 6.0;
        }
        
        return m1;
    }
}

/// Converts an HSL color string to a slice of R, G, B color values as u8 integers.
pub fn hsl(hsl: &str) -> Option<([u8; SLICE_LENGTH], Alpha)> {
    let mut s = hsl;
    let mut len = s.len();
    
    if len > 1 && s.starts_with("hsl(") {
        s = &s[4..len];
        len -= 4;
    } else if len > 1 && s.starts_with("hsla(") {
        s = &s[5..len];
        len -= 5;
    } else {
        return None;
    }
    
    if len > 1 && s.ends_with(')') {
        s = &s[..(len - 1)];
    } else {
        return None;
    }
    
    let mut iter = s.split(',');
    let hue = iter.next()?.trim().parse::<u16>().ok()?;
    let hue = (((hue as f32 % 360.0) + 360.0) % 360.0) / 360.0;
    let saturation = helpers::parse_percent(iter.next()?)?;
    let lightness = helpers::parse_percent(iter.next()?)?;
    let alpha = if let Some(value) = iter.next() {
        helpers::fit_percent(value.trim().parse::<f32>().ok()?)
    } else {
        1.0
    };
    let m2 = if lightness <= 0.5 {
        lightness * (saturation + 1.0)
    } else {
        lightness + saturation - lightness * saturation
    };
    let m1 = lightness * 2.0 - m2;
    let r = helpers::float_to_value(helpers::hue_to_rgb(m1, m2, hue + 1.0 / 3.0) * 255.0);
    let g = helpers::float_to_value(helpers::hue_to_rgb(m1, m2, hue) * 255.0);
    let b = helpers::float_to_value(helpers::hue_to_rgb(m1, m2, hue - 1.0 / 3.0) * 255.0);
    
    Some(([r, g, b], alpha))
}

/// Attempts to parse a hexadecimal color string into a color.
pub fn hex(hex: &str) -> Option<[u8; SLICE_LENGTH]> {
    let mut s = hex;
    let mut len = s.len();
    
    if len > 1 && s.starts_with('#') {
        s = &s[1..len];
        len -= 1;
    }
    
    if len == 3 {
        let mut colors = [0u8; SLICE_LENGTH];
        
        for i in 0..SLICE_LENGTH {
            let c = &s[i..(i + 1)];
            let c = [c, c].concat();
            let value = u8::from_str_radix(&c, 16).ok()?;
            
            colors[i] = value;
        }
        
        return Some(colors);
    }
    
    if len == 6 || len == 8 {
        let mut colors = [0u8; SLICE_LENGTH];
        
        for i in 0..SLICE_LENGTH {
            let j = i * 2;
            let value = u8::from_str_radix(&s[j..(j + 2)], 16).ok()?;
            
            colors[i] = value;
        }
        
        if len == 8 {
            // Expect the alpha to be a valid value
            u8::from_str_radix(&s[6..8], 16).ok()?;
        }
        
        return Some(colors);
    }
    
    None
}

/// Attempts to parse an rgb or rgba color string into a color. Alpha value defaults to `1.0` if 
/// not present.
pub fn rgba(rgb: &str) -> Option<([u8; SLICE_LENGTH], Alpha)> {
    let mut s = rgb;
    let mut len = s.len();
    let mut colors_expected = SLICE_LENGTH;
    
    if len > 1 && s.starts_with("rgb(") {
        s = &s[4..len];
        len -= 4;
    } else if len > 1 && s.starts_with("rgba(") {
        s = &s[5..len];
        len -= 5;
        colors_expected += 1;
    } else {
        return None;
    }
    
    if len > 1 && s.ends_with(')') {
        s = &s[..(len - 1)];
    } else {
        return None;
    }
    
    let mut colors = [0u8; 3];
    let mut num_colors = 0;
    let mut alpha: Alpha = 1.0;
    
    for (i, c) in s.split(',').enumerate() {
        if i >= colors_expected {
            return None;
        }
        
        match i {
            0..=2 => colors[i] = u8::from_str_radix(c.trim(), 10).ok()?,
            3 if colors_expected == 4 => if let Ok(value) = u8::from_str_radix(c.trim(), 10) {
                alpha = value as f32 / Value::MAX as Alpha;
            } else {
                alpha = c.trim().parse::<Alpha>().ok()?;
            }
            // Too many colors - invalid color
            _ => return None,
        }
        
        num_colors += 1;
    }
    
    // Check if the number of colors is valid.
    if num_colors != colors_expected {
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
    }
}