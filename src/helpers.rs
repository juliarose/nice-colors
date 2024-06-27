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

/// Covnerts an rgb color to HSL
pub fn rgb_to_hsl(
    r: Value,
    g: Value,
    b: Value,
) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let mut hue = (max + min) / 2.0;
    #[allow(unused_assignments)]
    let mut saturation = hue;
    let lightness = hue;
    
    if max == min {
        hue = 0.0;
        saturation = 0.0;
    } else {
        let difference = max - min;
        
        saturation = if lightness > 0.5 {
            difference / (2.0 - max - min)
        } else {
            difference / (max + min)
        };
        
        hue = if max == r {
            (g - b) / difference + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / difference + 2.0
        } else {
            (r - g) / difference + 4.0
        };
        hue = hue / 6.0;
    }
    
    (hue * 360.0, saturation, lightness)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn converts_rgb_to_hsl() {
        let (hue, saturation, lightness) = rgb_to_hsl(255, 0, 0);
        
        assert_eq!(hue, 0.0);
        assert_eq!(saturation, 1.0);
        assert_eq!(lightness, 0.5);
    }
    
    #[test]
    fn converts_rgb_to_hsl_periwinkle() {
        let (hue, saturation, lightness) = rgb_to_hsl(204, 204, 255);
        
        assert_eq!(hue, 240.0);
        assert_eq!(saturation, 1.0);
        assert_eq!(lightness, 0.9);
    }
}