use crate::{parse, html};
use std::fmt;
use std::hash::Hash;
use std::fmt::Write;

use crate::{Value, DecimalValue, Alpha, SLICE_LENGTH};

/// A color containing values for red, green, blue, and alpha.
pub type ColorWithAlpha = (Color, Alpha);

/// A color containing values for red, green, and blue.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct Color {
    /// The red value.
    pub red: Value,
    /// The green value.
    pub green: Value,
    /// The blue value.
    pub blue: Value,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format!("#{}", self.to_hex_string()))
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
    /// Creates a new [`Color`]. This defaults to black and is equivalent to [`Color::default()`].
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let color = Color::new();
    /// 
    /// assert_eq!(color, Color { red: 0, green: 0, blue: 0 });
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
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
    /// assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
    /// ```
    pub fn from_decimal(decimal: DecimalValue) -> Self {
        let bytes = decimal.to_be_bytes();
        
        Self {
            red: bytes[1],
            green: bytes[2],
            blue: bytes[3],
        }
    }
    
    /// Maps each value in this color.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let red = Color { red: 255, green: 0, blue: 0 };
    /// let mapped = red.map(|c| c / 2);
    /// 
    /// assert_eq!(mapped, Color { red: 127, green: 0, blue: 0 });
    /// ```
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(Value) -> Value,
    {
        let mut mapped = Color::default();
        
        mapped.red = f(self.red);
        mapped.green = f(self.green);
        mapped.blue = f(self.blue);
        mapped
    }
    
    /// Maps each value in this color with another color.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let red = Color { red: 255, green: 0, blue: 0 };
    /// let blue = Color { red: 0, green: 0, blue: 255 };
    /// let mapped = red.map_with(blue, |a, b| std::cmp::max(a, b));
    /// 
    /// assert_eq!(mapped, Color { red: 255, green: 0, blue: 255 });
    /// ```
    pub fn map_with<F>(&self, other: Self, f: F) -> Self
    where
        F: Fn(Value, Value) -> Value,
    {
        let mut mapped = Color::default();
        
        mapped.red = f(self.red, other.red);
        mapped.green = f(self.green, other.green);
        mapped.blue = f(self.blue, other.blue);
        mapped
    }
    
    /// Blends two colors.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let red = Color { red: 255, green: 0, blue: 0 };
    /// let blue = Color { red: 0, green: 0, blue: 255 };
    /// let amount = 0.5;
    /// let blended = red.blend(blue, amount);
    /// 
    /// assert_eq!(blended, Color { red: 128, green: 0, blue: 128 });
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
    /// let color = Color { red: 100, green: 100, blue: 100 };
    ///     
    /// assert_eq!(color.to_decimal(), 6579300);
    /// ```
    pub fn to_decimal(&self) -> DecimalValue {
        DecimalValue::from_le_bytes([self.red, self.green, self.blue, 0])
    }
    
    /// Converts this color into a hexadecimal color string.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color { red: 255, green: 0, blue: 0 }.to_hex_string(), "FF0000");
    /// ```
    pub fn to_hex_string(&self) -> String {
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
    /// assert_eq!(Color { red: 255, green: 0, blue: 0 }.to_rgba_string(0.5), "rgba(255,0,0,0.5)");
    /// ```
    pub fn to_rgba_string(&self, alpha: Alpha) -> String {
        let alpha = if alpha > 1.0 {
            1.0
        } else if alpha < 0.0 {
            0.0
        } else {
            alpha
        };
        
        format!("rgba({},{},{},{})", self.red, self.green, self.blue, alpha)
    }
    
    /// Converts this color into an rgb color string.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color { red: 255, green: 0, blue: 0 }.to_rgb_string(), "rgb(255,0,0)");
    /// ```
    pub fn to_rgb_string(&self) -> String {
        format!("rgb({},{},{})", self.red, self.green, self.blue)
    }
    
    /// Attempts to parse an rgb or rgba color string into a color. Ignores the alpha value if 
    /// present.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// let color = Color::from_rgb_str("rgb(100,100,100)").unwrap();
    /// 
    /// assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
    /// ````
    pub fn from_rgb_str(rgb: &str) -> Option<Self> {
        parse::rgba(rgb).map(|(colors, _alpha)| colors.into())
    }
    
    /// Attempts to parse an rgb or rgba color string into a color. Alpha defaults to `1.0` if not 
    /// present.
    pub fn from_rgba_str(rgb: &str) -> Option<ColorWithAlpha> {
        parse::rgba(rgb).map(|(colors, alpha)| (colors.into(), alpha))
    }
    
    /// Attempts to parse a hexadecimal color string into a color. Since this is explicitly 
    /// converting from a hexadecimal string, the hash symbol is optional.
    /// 
    /// However if you try converting a string using [`std::std::FromStr`], the hash symbol is 
    /// required.
    /// 
    /// # Examples
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::from_hex_str("FF0000").unwrap(), Color { red: 255, green: 0, blue: 0 });
    /// assert_eq!(Color::from_hex_str("#FF0000").unwrap(), Color { red: 255, green: 0, blue: 0 });
    /// assert_eq!(Color::from_hex_str("F00").unwrap(), Color { red: 255, green: 0, blue: 0 });
    /// ```
    pub fn from_hex_str(hex: &str) -> Option<Self> {
        parse::hex(hex, false).map(|colors| colors.into())
    }
    
    /// Attempts to parse an hsl color string into a color.
    pub fn from_hsl_str(hsl: &str) -> Option<Self> {
        parse::hsl(hsl).map(|(colors, _alpha)| colors.into())
    }
    
    /// Attempts to parse an hsl color string into a color with alpha.
    pub fn from_hsla_str(hsl: &str) -> Option<ColorWithAlpha> {
        parse::hsl(hsl).map(|(colors, alpha)| (colors.into(), alpha))
    }
    
    /// Converts this color into an array.
    pub fn to_array(&self) -> [Value; SLICE_LENGTH] {
        [
            self.red,
            self.green,
            self.blue,
        ]
    }
    
    /// Converts a slice into a color.
    fn from_slice(slice: [Value; SLICE_LENGTH]) -> Self {
        Self {
            red: slice[0],
            green: slice[1],
            blue: slice[2],
        }
    }
}

impl IntoIterator for Color {
    type Item = Value;
    type IntoIter = std::array::IntoIter<Value, SLICE_LENGTH>;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.to_array())
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

impl Into<[Value; SLICE_LENGTH]> for Color {
    fn into(self) -> [Value; SLICE_LENGTH] {
        self.to_array()
    }
}

impl Into<[Value; SLICE_LENGTH]> for &Color {
    fn into(self) -> [Value; SLICE_LENGTH] {
        self.to_array()
    }
}

impl From<(Value, Value, Value)> for Color {
    fn from(value: (Value, Value, Value)) -> Self {
        Self {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}

impl From<&(Value, Value, Value)> for Color {
    fn from(value: &(Value, Value, Value)) -> Self {
        Self {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}

impl From<Color> for (Value, Value, Value) {
    fn from(value: Color) -> Self {
        (value.red, value.green, value.blue)
    }
}

impl From<&Color> for (Value, Value, Value) {
    fn from(value: &Color) -> Self {
        (value.red, value.green, value.blue)
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

impl From<i32> for Color {
    fn from(value: i32) -> Self {
        Self::from_decimal(value as DecimalValue)
    }
}

impl From<&i32> for Color {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl From<Color> for DecimalValue {
    fn from(value: Color) -> Self {
        value.to_decimal()
    }
}

impl From<Color> for i32 {
    fn from(value: Color) -> Self {
        value.to_decimal() as i32
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_string())
    }
}

impl std::str::FromStr for Color {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(color) = parse::hex(s, true).map(|colors| colors.into()) {
            return Ok(color);
        }
        
        if let Some(color) = Self::from_rgb_str(s) {
            return Ok(color);
        }
        
        if let Some(color) = Self::from_hsl_str(s) {
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
    
    #[test]
    fn blends() {
        let a = Color { red: 0, green: 0, blue: 0 };
        let b = Color { red: 100, green: 100, blue: 100 };
        
        assert_eq!(a.blend(b, 0.5), Color { red: 50, green: 50, blue: 50 });
        assert_eq!(a.blend(b, 1.0), Color { red: 100, green: 100, blue: 100 });
        assert_eq!(a.blend(b, -100.0), Color { red: 0, green: 0, blue: 0 });
        assert_eq!(a.blend(b, 100.0), Color { red: 100, green: 100, blue: 100 });
    }
    
    #[test]
    fn converts_to_string() {
        let red = Color { red: 255, green: 0, blue: 0 };
        
        assert_eq!(red.to_string(), "FF0000");
    }
    
    #[test]
    fn converts_to_hex() {
        let red = Color { red: 255, green: 0, blue: 0 };
        
        assert_eq!(red.to_hex_string(), "FF0000");
    }
    
    #[test]
    fn converts_to_rgb() {
        let red = Color { red: 255, green: 0, blue: 0 };
        
        assert_eq!(red.to_rgb_string(), "rgb(255,0,0)");
    }
    
    #[test]
    fn converts_from_hex() {
        let red = Color { red: 255, green: 0, blue: 0 };
            
        assert_eq!(Color::from_hex_str("FF0000").unwrap(), red);
        assert_eq!(Color::from_hex_str("F00").unwrap(), red);
    }
    
    #[test]
    fn converts_from_slice() {
        let color = Color::from([255, 0, 0]);
        
        assert_eq!(color, Color { red: 255, green: 0, blue: 0 });
    }
    
    #[test]
    fn converts_from_str_errors_when_hash_symbol_is_not_present() {
        assert!("FF0000".parse::<Color>().is_err());
    }
    
    #[test]
    fn converts_from_str() {
        let color = "#FF0000".parse::<Color>().unwrap();
        
        assert_eq!(color, Color { red: 255, green: 0, blue: 0 });
    }
    
    #[test]
    fn converts_from_str_800080() {
        let color = "#800080".parse::<Color>().unwrap();
        
        assert_eq!(color, Color { red: 128, green: 0, blue: 128 });
    }
    
    #[test]
    fn converts_to_decimal() {
        let color = Color { red: 100, green: 100, blue: 100 };
        
        assert_eq!(color.to_decimal(), 6579300);
    }
    
    #[test]
    fn converts_from_decimal() {
        let color = Color::from_decimal(6579300);
        
        assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
    }
    
    #[test]
    fn converts_from_hex_color() {
        let color = Color::from(0xFF0000);
        
        assert_eq!(color, Color { red: 255, green: 0, blue: 0 });
        
        let color = Color::from_decimal(0xFF0000);
        
        assert_eq!(color, Color { red: 255, green: 0, blue: 0 });
    }
    
    #[test]
    fn converts_from_hex_color_0x112233() {
        let color = Color::from(0x112233);
        
        assert_eq!(color, Color { red: 17, green: 34, blue: 51 });
    }
    
    #[test]
    fn converts_to_from_decimal() {
        let color = Color { red: 100, green: 100, blue: 100 };
        let decimal = color.to_decimal();
        let color = Color::from_decimal(decimal);
        
        assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
    }
    
    #[test]
    fn converts_to_decimal_into() {
        let color = Color { red: 100, green: 100, blue: 100 };
        let decimal: DecimalValue = color.into();
        
        assert_eq!(decimal, 6579300);
    }
    
    #[test]
    fn converts_from_rgb() {
        let color = Color::from_rgb_str("rgb(100,100,100)").unwrap();
        
        assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
        
        let color = Color::from_rgb_str("rgb( 100, 100, 100 )").unwrap();
        
        assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
        
        let color = Color::from_rgb_str("rgba( 100, 100, 100, 1.0 )").unwrap();
        
        assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
    }
    
    #[test]
    fn converts_from_rgba() {
        let (color, alpha) = Color::from_rgba_str("rgba(100,100,100,0.5)").unwrap();
        
        assert_eq!(color, Color { red: 100, green: 100, blue: 100 });
        assert_eq!(alpha, 0.5);
        
        let (color, alpha) = Color::from_rgba_str("rgba(255,0,0,0.2)").unwrap();
        
        assert_eq!(color, Color { red: 255, green: 0, blue: 0 });
        assert_eq!(alpha, 0.2);
    }
}