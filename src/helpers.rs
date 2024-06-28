use crate::color::Value;

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
    value.max(0.0).min(1.0)
}

/// Parses a percentage value from a string.
pub fn parse_percent(s: &str) -> Option<f32> {
    if s.ends_with('%') {
        let value = remove_suffix(s.trim(), "%")?.parse::<f32>().ok()?;
        let percent = fit_percent(value / 100.0);
        
        return Some(percent);
    } else if s.starts_with("0.") || s.starts_with('.') {
        let percent = fit_percent(s.parse::<f32>().ok()?);
        
        return Some(percent);
    }
    
    None
}

/// Converts a floating point value to a percentage string.
pub fn float_to_percent(value: f32) -> f32 {
    let percent = value * 100.0;
    // Keep only 3 decimal places.
    let rounded = (percent * 1000.0).round() / 1000.0;
    
    return rounded;
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

pub mod conversions {
    use super::*;
    
    /// Converts a hue to RGB.
    pub fn hue_to_rgb(m1: f32, m2: f32, mut hue: f32) -> f32 {
        // Sourced from: https://github.com/7thSigil/css-color-parser-rs/blob/v0.1.2/src/color/color.rs#L366
        if hue < 0.0 {
            hue += 1.0;
        } else if hue > 1.0 {
            hue -= 1.0;
        }
        
        if hue < 1.0 / 6.0 {
            return m1 + (m2 - m1) * hue * 6.0;
        }
        
        if hue < 1.0 / 2.0 {
            return m2;
        }
        
        if hue < 2.0 / 3.0 {
            return m1 + (m2 - m1) * (2.0 / 3.0 - hue) * 6.0;
        }
        
        return m1;
    }
    
    /// Converts an rgb color to HSL
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
    
    pub fn hsl_to_rgb(
        mut hue: f32,
        mut saturation: f32,
        mut lightness: f32,
    ) -> (Value, Value, Value) {
        hue = hue.max(0.0).min(360.0);
        hue = hue / 360.0;
        saturation = fit_percent(saturation);
        lightness = fit_percent(lightness);
        
        let m2 = if lightness <= 0.5 {
            lightness * (saturation + 1.0)
        } else {
            lightness + saturation - lightness * saturation
        };
        let m1 = lightness * 2.0 - m2;
        let r = float_to_value(hue_to_rgb(m1, m2, hue + 1.0 / 3.0) * 255.0);
        let g = float_to_value(hue_to_rgb(m1, m2, hue) * 255.0);
        let b = float_to_value(hue_to_rgb(m1, m2, hue - 1.0 / 3.0) * 255.0);
        
        (r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn converts_rgb_to_hsl() {
        let (hue, saturation, lightness) = conversions::rgb_to_hsl(255, 0, 0);
        
        assert_eq!(hue, 0.0);
        assert_eq!(saturation, 1.0);
        assert_eq!(lightness, 0.5);
    }
    
    #[test]
    fn converts_rgb_to_hsl_periwinkle() {
        let (hue, saturation, lightness) = conversions::rgb_to_hsl(204, 204, 255);
        
        assert_eq!(hue, 240.0);
        assert_eq!(saturation, 1.0);
        assert_eq!(lightness, 0.9);
    }
    
    #[test]
    fn converts_hsl_to_rgb() {
        let (r, g, b) = conversions::hsl_to_rgb(0.0, 1.0, 0.5);
        
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
        
        let (h, s, l) = conversions::rgb_to_hsl(r, g, b);
        
        assert_eq!(h, 0.0);
        assert_eq!(s, 1.0);
        assert_eq!(l, 0.5);
    }
    
    #[test]
    fn converts_hsl_to_rgb_2() {
        let (r, g, b) = conversions::hsl_to_rgb(340.0, 0.5, 0.5);
        
        assert_eq!(r, 191);
        assert_eq!(g, 64);
        assert_eq!(b, 106);
    }
}