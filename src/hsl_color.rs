use crate::Color;
use crate::helpers::conversions;

/// A color containing values for hue, saturation, and lightness.
#[derive(Debug, Clone, Copy, PartialEq, Default, PartialOrd)]
pub struct HSLColor {
    /// The hue value.
    pub hue: f32,
    /// The saturation value.
    pub saturation: f32,
    /// The lightness value.
    pub lightness: f32,
}

impl HSLColor {
    /// Creates a new HSL color.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sets the hue value.
    /// 
    /// The hue value is a float between 0.0 and 360.0.
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
    
    /// Sets the saturation value.
    pub fn saturation(self, mut saturation: f32) -> Self {
        saturation = saturation.max(0.0).min(1.0);
        
        Self { saturation, ..self }
    }
    
    /// Sets the lightness value.
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
