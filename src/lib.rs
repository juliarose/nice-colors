//! # nice-colors
//! 
//! Lightweight module for working with colors. The aim is to provide a wrapper for RGB color 
//! values in a way that does not compromise performance. Colors are packed as three `u8` values to
//! provide as lean of a memory footprint as possible. While there are also some helpers for 
//! working with alpha values, they are not a main feature of this module.

#![warn(missing_docs)]

#[cfg(feature = "serde")]
pub mod serializers;
pub mod html;
mod parse;

use std::fmt;
use std::hash::Hash;
use std::fmt::Write;

/// A color containing values for red, green, blue, and alpha.
pub type ColorWithAlpha = (Color, Alpha);

pub(crate) type Value = u8;
pub(crate) type Alpha = f32;
pub(crate) type DecimalValue = u32;

pub(crate) const SLICE_LENGTH: usize = 3;

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
        deserializer.deserialize_any(crate::serializers::ColorVisitor)
    }
}

impl Color {
    /// Creates a new [`Color`].
    pub fn new(
        r: Value,
        g: Value,
        b: Value,
    ) -> Self {
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
        DecimalValue::from_le_bytes([self.r, self.g, self.b, 0])
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
        
        format!("rgba({},{},{},{})", self.r, self.g, self.b, alpha)
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
        format!("rgb({},{},{})", self.r, self.g, self.b)
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
        parse::rgba(rgb).map(|(colors, _alpha)| colors.into())
    }
    
    /// Attempts to parse an rgb or rgba color string into a color. Alpha defaults to `1.0` if not 
    /// present.
    pub fn from_rgba(rgb: &str) -> Option<ColorWithAlpha> {
        parse::rgba(rgb).map(|(colors, alpha)| (colors.into(), alpha))
    }
    
    /// Attempts to parse a hexadecimal color string into a color.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::from_hex("FF0000").unwrap(), Color::new(255, 0, 0));
    /// assert_eq!(Color::from_hex("F00").unwrap(), Color::new(255, 0, 0));
    /// ```
    pub fn from_hex(hex: &str) -> Option<Self> {
        parse::hex(hex).map(|colors| colors.into())
    }
    
    /// Attempts to parse an hsl color string into a color.
    pub fn from_hsl(hsl: &str) -> Option<Self> {
        parse::hsl(hsl).map(|(colors, _alpha)| colors.into())
    }
    
    /// Attempts to parse an hsl color string into a color with alpha.
    pub fn from_hsla(hsl: &str) -> Option<ColorWithAlpha> {
        parse::hsl(hsl).map(|(colors, alpha)| (colors.into(), alpha))
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
        if let Some(color) = Self::from_hex(s) {
            return Ok(color);
        }
        
        if let Some(color) = Self::from_rgb(s) {
            return Ok(color);
        }
        
        if let Some(color) = Self::from_hsl(s) {
            return Ok(color);
        }
        
        if let Some(color) = html::from_html_color_name(s) {
            return Ok(color);
        }
        
        return Err("Not a valid color string.");
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
    fn converts_from_str_with_pound_symbol() {
        let color = Color::from_str("#FF0000").unwrap();
        
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
        let decimal: DecimalValue = color.into();
        
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
        assert_eq!(alpha, 0.5);
        
        let (color, alpha) = Color::from_rgba("rgba(255,0,0,0.2)").unwrap();
        
        assert_eq!(color, Color::new(255, 0, 0));
        assert_eq!(alpha, 0.2);
    }
}