
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