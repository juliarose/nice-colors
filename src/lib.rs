use std::fmt;

type Value = u8;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Color(pub [Value; 3]);

impl Color {
    pub fn new(r: Value, g: Value, b: Value) -> Self {
        Self([r, g, b])
    }
    
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(Value) -> Value,
    {
        let mut mapped = Color::default();
        
        for i in 0..3 {
            mapped.0[i] = f(self.0[i]);
        }
        
        mapped
    }
    
    fn map_with<F>(&self, other: Self, f: F) -> Self
    where
        F: Fn(Value, Value) -> Value,
    {
        let mut mapped = Color::default();
        
        for i in 0..3 {
            mapped.0[i] = f(self.0[i], other.0[i]);
        }
        
        mapped
    }
    
    /// Blends two colors
    /// 
    /// # Examples
    /// 
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
    pub fn blend(&self, other: Color, amount: f32) -> Color {
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
    
    /// Converts this color into a hexadecimal color string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::new(255, 0, 0).to_hex(), "FF0000");
    /// ```
    pub fn to_hex(&self) -> String {
        self
            .into_iter()
            .map(|v| format!("{v:02X}"))
            .collect()
    }
    
    /// Converts this color into an rgba color string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use nice_colors::Color;
    /// 
    /// assert_eq!(Color::new(255, 0, 0).to_rgba(0.5), "rgba(255,0,0,0.5)");
    /// ```
    pub fn to_rgba(&self, alpha: f32) -> String {
        let values = self
            .into_iter()
            .map(|v| v.to_string())
            .chain([print_float(alpha)])
            .collect::<Vec<_>>()
            .join(",");
        
        format!("rgba({values})")
    }
    
    /// Converts this color into an rgb color string.
    /// 
    /// # Examples
    /// 
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
    /// 
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
            for i in 0..3 {
                let c = &s[i..(i + 1)];
                let c = [c, c].concat();
                
                color.0[i] = u8::from_str_radix(&c, 16).ok()?;
            }
            
            return Some(color);
        }
        
        if len == 6 {
            for i in 0..3 {
                let ii = i * 2;
                
                color.0[i] = u8::from_str_radix(&s[ii..(ii + 2)], 16).ok()?;
            }
            
            return Some(color);
        }
        
        None
    }
}

impl IntoIterator for Color {
    type Item = Value;
    type IntoIter = std::array::IntoIter<Value, 3>;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

impl From<[Value; 3]> for Color {
    fn from(value: [Value; 3]) -> Self {
        Self(value)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[derive(Debug)]
pub struct ColorFromStrError;

impl fmt::Display for ColorFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not a valid color string.")
    }
}

impl std::str::FromStr for Color {
    type Err = ColorFromStrError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s).ok_or(ColorFromStrError)
    }
}

fn print_float(amount: f32) -> String {
    ((amount * 1_000.0).round() / 1_000.0).to_string()
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
}