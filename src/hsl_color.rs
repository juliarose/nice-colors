use crate::Color;
use crate::helpers::conversions;

/// A color containing values for hue, saturation, and lightness.
#[derive(Debug, Clone, Copy, PartialEq, Default, PartialOrd)]
pub struct HSLColor {
    /// The hue value (0.0 to 360.0).
    pub hue: f32,
    /// The saturation value (0.0 to 1.0).
    pub saturation: f32,
    /// The lightness value (0.0 to 1.0).
    pub lightness: f32,
}

impl HSLColor {
    /// Creates a new HSL color.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets the hue value.
    /// 
    /// The hue value is a float between 0.0 and 360.0:
    /// - If the value is less than 0.0, it will be set to 0.0.
    /// - If the value is greater than 360.0, it will be set to 360.0.
    /// 
    /// # Example
    /// ```
    /// use nice_colors::HSLColor;
    /// 
    /// let color = HSLColor {
    ///     hue: 0.0,
    ///     saturation: 0.5,
    ///     lightness: 0.5,
    /// };
    /// let color = color.hue(180.0);
    /// 
    /// assert_eq!(color.hue, 180.0);
    /// ```
    pub fn hue(self, mut hue: f32) -> Self {
        hue = hue.max(0.0).min(360.0);
        
        Self { hue, ..self }
    }
    
    /// Rotates the hue value by a specified amount.
    pub fn rotate_hue(self, mut hue: f32) -> Self {
        if self.hue + hue > 360.0 {
            hue = self.hue + hue - 360.0;
        } else if self.hue + hue < 0.0 {
            hue = self.hue + hue + 360.0;
        } else {
            hue = self.hue + hue;
        }
        
        Self { hue, ..self }
    }
    
    /// Sets the saturation value.
    /// 
    /// The saturation value is a float between 0.0 and 1.0:
    /// - If the value is less than 0.0, it will be set to 0.0.
    /// - If the value is greater than 1.0, it will be set to 1.0.
    pub fn saturation(self, mut saturation: f32) -> Self {
        saturation = saturation.max(0.0).min(1.0);
        
        Self { saturation, ..self }
    }
    
    /// Sets the lightness value.
    /// 
    /// The lightness value is a float between 0.0 and 1.0:
    /// - If the value is less than 0.0, it will be set to 0.0.
    /// - If the value is greater than 1.0, it will be set to 1.0.
    pub fn lightness(self, mut lightness: f32) -> Self {
        lightness = lightness.max(0.0).min(1.0);
        
        Self { lightness, ..self }
    }
}

impl From<Color> for HSLColor {
    fn from(color: Color) -> Self {
        let (
            hue,
            saturation,
            lightness,
        ) = conversions::rgb_to_hsl(
            color.red,
            color.green,
            color.blue,
        );
        
        Self {
            hue,
            saturation,
            lightness,
        }
    }
}

impl From<&Color> for HSLColor {
    fn from(color: &Color) -> Self {
        Self::from(*color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn rotates_hue() {
        let color = HSLColor {
            hue: 10.0,
            saturation: 0.5,
            lightness: 0.5,
        };
        let color = color.rotate_hue(180.0);
        
        assert_eq!(color.hue, 190.0);
    }
}