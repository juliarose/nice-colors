//! # nice-colors
//! 
//! Provides a [`Color`] type that represents a color with RGB color values along with methods 
//! commonly used for manipulating colors.

#![warn(missing_docs)]
use std::fmt;
use std::hash::Hash;
use std::fmt::Write;

type Value = u8;
type DecimalValue = i32;

const SLICE_LENGTH: usize = 3;

/// A color containing values for red, green, and blue.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct Color(pub [Value; SLICE_LENGTH]);

impl Color {
    /// Creates a new [`Color`].
    pub fn new(r: Value, g: Value, b: Value) -> Self {
        Self([r, g, b])
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
        
        Self([bytes[0], bytes[1], bytes[2]])
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
        
        for i in 0..SLICE_LENGTH {
            mapped.0[i] = f(self.0[i]);
        }
        
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
        
        for i in 0..SLICE_LENGTH {
            mapped.0[i] = f(self.0[i], other.0[i]);
        }
        
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
    pub fn blend(&self, other: Color, amount: f32) -> Self {
        if amount >= 1.0 {
            return other;
        }
        
        if amount <= 0.0 {
            return *self;
        }
        
        self.map_with(other, |a, b| {
            let a = a as f32 * (1.0 - amount);
            let b = b as f32 * amount;
            
            (a + b).round() as u8
        })
    }
    
    /// Gets the red color value.
    pub fn red(&self) -> Value {
        self.0[0]
    }
    
    /// Gets the green color value.
    pub fn green(&self) -> Value {
        self.0[1]
    }
    
    /// Gets the blue color value.
    pub fn blue(&self) -> Value {
        self.0[2]
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
        i32::from_le_bytes([self.0[0], self.0[1], self.0[2], 0])
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
    pub fn to_rgba(&self, alpha: f32) -> String {
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
        
        let mut color = Color::default();
        
        if len == 3 {
            for i in 0..SLICE_LENGTH {
                let c = &s[i..(i + 1)];
                let c = [c, c].concat();
                
                color.0[i] = u8::from_str_radix(&c, 16).ok()?;
            }
            
            return Some(color);
        }
        
        if len == 6 {
            for i in 0..SLICE_LENGTH {
                let j = i * 2;
                
                color.0[i] = u8::from_str_radix(&s[j..(j + 2)], 16).ok()?;
            }
            
            return Some(color);
        }
        
        None
    }
    
    /// Gets the inner slice of this color.
    pub fn as_slice(&self) -> &[Value; SLICE_LENGTH] {
        &self.0
    }
    
    /// Converts the inner slice of this color into a vector.
    pub fn to_vec(&self) -> Vec<Value> {
        self.0.to_vec()
    }
}

impl IntoIterator for Color {
    type Item = Value;
    type IntoIter = std::array::IntoIter<Value, SLICE_LENGTH>;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

impl From<[Value; SLICE_LENGTH]> for Color {
    fn from(value: [Value; SLICE_LENGTH]) -> Self {
        Self(value)
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
}