//! # nice-colors
//! 
//! Provides a [`Color`] type that represents a color with RGB color values along with methods 
//! commonly used for manipulating colors.

#![warn(missing_docs)]

#[cfg(feature = "serde")]
pub mod serializers;
pub mod html;

use std::fmt;
use std::hash::Hash;
use std::fmt::Write;

type Value = u8;
type Alpha = f32;
type DecimalValue = i32;

const SLICE_LENGTH: usize = 3;

/// A color containing values for red, green, and blue.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct Color {
    /// The red value.
    pub r: Value,
    /// The green value.
    pub b: Value,
    /// The blue value.
    pub g: Value,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format!("#{}", self.to_hex()))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColorVisitor;
        
        impl<'de> serde::de::Visitor<'de> for ColorVisitor {
            type Value = Color;
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hexadecimal or rgb color string")
            }
            
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.starts_with("rgb") {
                    return Color::from_rgb(v).ok_or(serde::de::Error::custom("Not a valid rgb color string."));
                }
                
                v.parse::<Color>().map_err(serde::de::Error::custom)
            }
        }
        
        deserializer.deserialize_str(ColorVisitor)
    }
}

impl Color {
    /// Creates a new [`Color`].
    pub fn new(r: Value, g: Value, b: Value) -> Self {
        Self {
            r,
            g,
            b,
        }
    }
    
    /// Converts a decimal color value into a color.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let color = Color::from_decimal(6579300);
    /// 
    /// assert_eq!(color, Color::new(100, 100, 100));
    /// ```
    pub fn from_decimal(decimal: DecimalValue) -> Self {
        let bytes = decimal.to_le_bytes();
        
        Self {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
        }
    }
    
    /// Maps each value in this color.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let red = Color::new(255, 0, 0);
    /// let mapped = red.map(|c| c / 2);
    /// 
    /// assert_eq!(mapped, Color::new(127, 0, 0));
    /// ```
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(Value) -> Value,
    {
        let mut mapped = Color::default();
        
        mapped.r = f(self.r);
        mapped.g = f(self.g);
        mapped.b = f(self.b);
        mapped
    }
    
    /// Maps each value in this color with another color.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let red = Color::new(255, 0, 0);
    /// let blue = Color::new(0, 0, 255);
    /// let mapped = red.map_with(blue, |a, b| std::cmp::max(a, b));
    /// 
    /// assert_eq!(mapped, Color::new(255, 0, 255));
    /// ```
    pub fn map_with<F>(&self, other: Self, f: F) -> Self
    where
        F: Fn(Value, Value) -> Value,
    {
        let mut mapped = Color::default();
        
        mapped.r = f(self.r, other.r);
        mapped.g = f(self.g, other.g);
        mapped.b = f(self.b, other.b);
        mapped
    }
    
    /// Blends two colors.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let red = Color::new(255, 0, 0);
    /// let blue = Color::new(0, 0, 255);
    /// let amount = 0.5;
    /// let blended = red.blend(blue, amount);
    /// 
    /// assert_eq!(blended, Color::new(128, 0, 128));
    /// ```
    pub fn blend(&self, other: Color, amount: Alpha) -> Self {
        if amount >= 1.0 {
            return other;
        }
        
        if amount <= 0.0 {
            return *self;
        }
        
        self.map_with(other, |a, b| {
            let a = a as Alpha * (1.0 - amount);
            let b = b as Alpha * amount;
            
            (a + b).round() as Value
        })
    }
    
    /// Converts this color into a decimal color value.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let color = Color::new(100, 100, 100);
    ///     
    /// assert_eq!(color.to_decimal(), 6579300);
    /// ```
    pub fn to_decimal(&self) -> DecimalValue {
        i32::from_le_bytes([self.r, self.g, self.b, 0])
    }
    
    /// Converts this color into a hexadecimal color string.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::new(255, 0, 0).to_hex(), "FF0000");
    /// ```
    pub fn to_hex(&self) -> String {
        self
            .into_iter()
            .fold(String::new(),|mut output, b| {
                let _ = write!(output, "{b:02X}");
                output
            })
    }
    
    /// Converts this color into an rgba color string.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::new(255, 0, 0).to_rgba(0.5), "rgba(255,0,0,0.5)");
    /// ```
    pub fn to_rgba(&self, alpha: Alpha) -> String {
        let alpha = if alpha > 1.0 {
            1.0
        } else if alpha < 0.0 {
            0.0
        } else {
            alpha
        };
        let values = self
            .into_iter()
            .map(|v| v.to_string())
            // Round alpha to 3 decimal places.
            .chain([((alpha * 1_000.0).round() / 1_000.0).to_string()])
            .collect::<Vec<_>>()
            .join(",");
        
        format!("rgba({values})")
    }
    
    /// Converts this color into an rgb color string.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::new(255, 0, 0).to_rgb(), "rgb(255,0,0)");
    /// ```
    pub fn to_rgb(&self) -> String {
        let values = self
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        
        format!("rgb({values})")
    }
    
    /// Attempts to parse an rgb or rgba color string into a color. Ignores the alpha value if 
    /// present.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let color = Color::from_rgb("rgb(100,100,100)").unwrap();
    /// 
    /// assert_eq!(color, Color::new(100, 100, 100));
    /// ````
    pub fn from_rgb(rgb: &str) -> Option<Self> {
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
        
        let mut color = Color::default();
        let mut num_colors = 0;
        
        for (i, c) in s.split(',').enumerate() {
            if i >= colors_expected {
                return None;
            }
            
            match i {
                0 => color.r = u8::from_str_radix(c.trim(), 10).ok()?,
                1 => color.g = u8::from_str_radix(c.trim(), 10).ok()?,
                2 => color.b = u8::from_str_radix(c.trim(), 10).ok()?,
                // We expect the alpha to be valid if it is included
                3 if colors_expected == 4 => if let Ok(_value) = u8::from_str_radix(c.trim(), 10) {
                    // It's a valid u8 integer
                } else {
                    c.trim().parse::<Alpha>().ok()?;
                },
                _ => return None,
            }
            
            num_colors += 1;
        }
        
        // Checks if the number of colors is valid.
        if num_colors != colors_expected {
            return None;
        }
        
        Some(color)
    }
    
    /// Converts this color from an rgb or rgba color string.
    pub fn from_rgba(rgb: &str) -> Option<(Self, Option<Alpha>)> {
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
        
        let mut color = Color::default();
        let mut num_colors = 0;
        let mut alpha: Option<Alpha> = None;
        
        for (i, c) in s.split(',').enumerate() {
            if i >= colors_expected {
                return None;
            }
            
            match i {
                0 => color.r = u8::from_str_radix(c.trim(), 10).ok()?,
                1 => color.g = u8::from_str_radix(c.trim(), 10).ok()?,
                2 => color.b = u8::from_str_radix(c.trim(), 10).ok()?,
                3 if colors_expected == 4 => if let Ok(value) = u8::from_str_radix(c.trim(), 10) {
                    alpha = Some(value as f32 / Value::MAX as Alpha);
                } else {
                    alpha = Some(c.trim().parse::<Alpha>().ok()?);
                }
                _ => return None,
            }
            
            num_colors += 1;
        }
        
        // Checks if the number of colors is valid.
        if num_colors != colors_expected {
            return None;
        }
        
        Some((color, alpha))
    }
    
    /// Converts this color from a hexadecimal color string.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::from_hex("FF0000").unwrap(), Color::new(255, 0, 0));
    /// assert_eq!(Color::from_hex("F00").unwrap(), Color::new(255, 0, 0));
    /// ```
    pub fn from_hex(hex: &str) -> Option<Self> {
        let mut s = hex;
        let mut len = s.len();
        
        if len > 1 && s.starts_with('#') {
            s = &s[1..len];
            len -= 1;
        }
        
        if len == 3 {
            let mut colors = [0u8; 3];
            
            for i in 0..SLICE_LENGTH {
                let c = &s[i..(i + 1)];
                let c = [c, c].concat();
                let value = u8::from_str_radix(&c, 16).ok()?;
                
                colors[i] = value;
            }
            
            return Some(Color::new(colors[0], colors[1], colors[2]));
        }
        
        if len == 6 || len == 8 {
            let mut colors = [0u8; 3];
            
            for i in 0..SLICE_LENGTH {
                let j = i * 2;
                let value = u8::from_str_radix(&s[j..(j + 2)], 16).ok()?;
                
                colors[i] = value;
            }
            
            if len == 8 {
                // Expect the alpha to be a valid value
                u8::from_str_radix(&s[6..8], 16).ok()?;
            }
            
            return Some(Color::new(colors[0], colors[1], colors[2]));
        }
        
        None
    }
    
    /// Converts this color into a slice.
    pub fn to_bytes(&self) -> [Value; SLICE_LENGTH] {
        [self.r, self.g, self.b]
    }
    
    /// Converts the inner slice of this color into a vector.
    pub fn to_vec(&self) -> Vec<Value> {
        self.to_bytes().to_vec()
    }
    
    /// Converts a slice into a color.
    pub fn from_slice(slice: [Value; SLICE_LENGTH]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }
}

impl IntoIterator for Color {
    type Item = Value;
    type IntoIter = std::array::IntoIter<Value, SLICE_LENGTH>;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.to_bytes())
    }
}

impl From<[Value; SLICE_LENGTH]> for Color {
    fn from(value: [Value; SLICE_LENGTH]) -> Self {
        Self::from_slice(value)
    }
}

impl From<&[Value; SLICE_LENGTH]> for Color {
    fn from(value: &[Value; SLICE_LENGTH]) -> Self {
        Self::from(*value)
    }
}

impl From<DecimalValue> for Color {
    fn from(value: DecimalValue) -> Self {
        Self::from_decimal(value)
    }
}

impl From<&DecimalValue> for Color {
    fn from(value: &DecimalValue) -> Self {
        Self::from_decimal(*value)
    }
}

impl From<Color> for DecimalValue {
    fn from(value: Color) -> Self {
        value.to_decimal()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl std::str::FromStr for Color {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s).ok_or("Not a valid color string.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    
    #[test]
    fn blends() {
        let a = Color::new(0, 0, 0);
        let b = Color::new(100, 100, 100);
        
        assert_eq!(a.blend(b, 0.5), Color::new(50, 50, 50));
        assert_eq!(a.blend(b, 1.0), Color::new(100, 100, 100));
        assert_eq!(a.blend(b, -100.0), Color::new(0, 0, 0));
        assert_eq!(a.blend(b, 100.0), Color::new(100, 100, 100));
    }
    
    #[test]
    fn converts_to_string() {
        let red = Color::new(255, 0, 0);
        
        assert_eq!(red.to_string(), "FF0000");
    }
    
    #[test]
    fn converts_to_hex() {
        let red = Color::new(255, 0, 0);
        
        assert_eq!(red.to_hex(), "FF0000");
    }
    
    #[test]
    fn converts_to_rgb() {
        let red = Color::new(255, 0, 0);
        
        assert_eq!(red.to_rgb(), "rgb(255,0,0)");
    }
    
    #[test]
    fn converts_from_hex() {
        let red = Color::new(255, 0, 0);
            
        assert_eq!(Color::from_hex("FF0000").unwrap(), red);
        assert_eq!(Color::from_hex("F00").unwrap(), red);
    }
    
    #[test]
    fn converts_from_slice() {
        let color = Color::from([255, 0, 0]);
        
        assert_eq!(color, Color::new(255, 0, 0));
    }
    
    #[test]
    fn converts_from_str() {
        let color = Color::from_str("FF0000").unwrap();
        
        assert_eq!(color, Color::new(255, 0, 0));
    }
    
    #[test]
    fn converts_to_decimal() {
        let color = Color::new(100, 100, 100);
        
        assert_eq!(color.to_decimal(), 6579300);
    }
    
    #[test]
    fn converts_from_decimal() {
        let color = Color::from_decimal(6579300);
        
        assert_eq!(color, Color::new(100, 100, 100));
    }
    
    #[test]
    fn converts_to_from_decimal() {
        let color = Color::new(100, 100, 100);
        let decimal = color.to_decimal();
        let color = Color::from_decimal(decimal);
        
        assert_eq!(color, Color::new(100, 100, 100));
    }
    
    #[test]
    fn converts_to_i32() {
        let color = Color::new(100, 100, 100);
        let decimal: i32 = color.into();
        
        assert_eq!(decimal, 6579300);
    }
    
    #[test]
    fn converts_from_rgb() {
        let color = Color::from_rgb("rgb(100,100,100)").unwrap();
        
        assert_eq!(color, Color::new(100, 100, 100));
        
        let color = Color::from_rgb("rgb( 100, 100, 100 )").unwrap();
        
        assert_eq!(color, Color::new(100, 100, 100));
        
        let color = Color::from_rgb("rgba( 100, 100, 100, 1.0 )").unwrap();
        
        assert_eq!(color, Color::new(100, 100, 100));
    }
    
    #[test]
    fn converts_from_rgba() {
        let (color, alpha) = Color::from_rgba("rgba(100,100,100,0.5)").unwrap();
        
        assert_eq!(color, Color::new(100, 100, 100));
        assert_eq!(alpha, Some(0.5));
        
        let (color, alpha) = Color::from_rgba("rgba(255,0,0,0.2)").unwrap();
        
        assert_eq!(color, Color::new(255, 0, 0));
        assert_eq!(alpha, Some(0.2));
    }
}