//! Serializers for the `Color` type.

use crate::{Color, ColorWithAlpha};
use serde::de;
use std::fmt;

/// Deserializes from hexademical and rgb color strings.
pub(crate) struct ColorVisitor;

impl<'de> de::Visitor<'de> for ColorVisitor {
    type Value = Color;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hexadecimal, rgb, or hsl color string")
    }
    
    /// Deserializes from a color string.
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        s.parse::<Self::Value>().map_err(serde::de::Error::custom)
    }
}

/// Deserializes from optional hexademical and rgb color strings.
struct OptionColorVisitor;

impl<'de> de::Visitor<'de> for OptionColorVisitor {
    type Value = Option<Color>;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hexadecimal or rgba color string or none")
    }
    
    /// Deserializes from a color string.
    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(ColorVisitor).map(Some)
    }
    
    /// Deserializes from a color string.
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
    
    /// Deserializes from a color string.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

/// Deserializes from hexademical and rgb color strings with alpha.
struct ColorAlphaVisitor;

impl<'de> de::Visitor<'de> for ColorAlphaVisitor {
    type Value = ColorWithAlpha;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A hexadecimal color string or none")
    }
    
    /// Deserializes from a color string.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.starts_with("rgb") {
            let (color, alpha) = Color::from_rgba(v)
                .ok_or(serde::de::Error::custom("Not a valid rgb color string."))?;
            
            return Ok((color, alpha));
        }
        
        v.parse::<Color>().map_err(serde::de::Error::custom).and_then(|color| {
            Ok((color, 1.0))
        })
    }
}

/// Deserializes from optional hexademical and rgb color strings with alpha.
struct OptionColorAlphaVisitor;

impl<'de> de::Visitor<'de> for OptionColorAlphaVisitor {
    type Value = Option<(Color, f32)>;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hexadecimal or rgba color string or none")
    }
    
    /// Deserializes from a color string.
    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(ColorAlphaVisitor).map(Some)
    }
    
    /// Deserializes from a color string.
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
    
    /// Deserializes from a color string.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

/// Serializes and deserializes to and from hexademical color strings. Deserialization also 
/// supports rgb color strings.
pub mod hex {
    use super::ColorVisitor;
    use crate::Color;
    use serde::{Serializer, Deserializer};
    
    /// Serializes a color to a hex string.
    pub fn serialize<S>(value: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format!("#{}", value.to_hex()))
    }
    
    /// Deserializes a color from a hex string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorVisitor)
    }
}

/// Serializes and deserializes to and from optional hexademical color strings. Deserialization 
/// also supports rgb color strings.
pub mod hex_option {
    use super::OptionColorVisitor;
    use crate::Color;
    use serde::{Serializer, Deserializer};
    
    /// Serializes a color to a hex string.
    pub fn serialize<S>(value: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            serializer.collect_str(&format!("#{}", value.to_hex()))
        } else {
            serializer.serialize_none()
        }
    }
    
    /// Deserializes a color from a hex string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionColorVisitor)
    }
}

/// Serializes and deserializes to and from rgb color strings. Deserialization also supports 
/// hexadecimal color strings.
pub mod rgb {
    use super::ColorVisitor;
    use crate::Color;
    use serde::{Serializer, Deserializer};
    
    /// Serializes a color to an rgb string.
    pub fn serialize<S>(value: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format!("{}", value.to_rgb()))
    }
    
    /// Deserializes a color from an rgb string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorVisitor)
    }
}

/// Serializes and deserializes to and from optional rgb color strings. Deserialization 
/// also supports hexadecimal color strings.
pub mod rgb_option {
    use super::OptionColorVisitor;
    use crate::Color;
    use serde::{Serializer, Deserializer};
    
    /// Serializes a color to an rgb string.
    pub fn serialize<S>(value: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            serializer.collect_str(&format!("{}", value.to_rgb()))
        } else {
            serializer.serialize_none()
        }
    }
    
    /// Deserializes a color from an rgb string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionColorVisitor)
    }
}

/// Serializes and deserializes to and from rgb color strings with alpha. Deserialization also 
/// supports hexadecimal color strings.
pub mod rgba {
    use super::ColorAlphaVisitor;
    use crate::Color;
    use serde::{Serializer, Deserializer};
    
    /// Serializes a color to an rgba string.
    pub fn serialize<S>(value: &(Color, f32), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if value.1 <= 1.0 {
            serializer.collect_str(&format!("{}", value.0.to_rgba(value.1)))
        } else {
            serializer.collect_str(&format!("{}", value.0.to_rgb()))
        }
    }
    
    /// Deserializes a color from an rgba string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<(Color, f32), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorAlphaVisitor)
    }
}

/// Serializes and deserializes to and from rgb color strings with alpha. Deserialization also 
/// supports hexadecimal color strings.
pub mod rgba_option {
    use super::OptionColorAlphaVisitor;
    use crate::{Color, ColorWithAlpha};
    use serde::{Serializer, Deserializer};
    
    /// Serializes a color to an rgba string.
    pub fn serialize<S>(value: &Option<(Color, f32)>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            if value.1 <= 1.0 {
                serializer.collect_str(&format!("{}", value.0.to_rgba(value.1)))
            } else {
                serializer.collect_str(&format!("{}", value.0.to_rgb()))
            }
        } else {
            serializer.serialize_none()
        }
    }
    
    /// Deserializes a color from an rgba string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ColorWithAlpha>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionColorAlphaVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    
    #[test]
    fn test_hex_serialize() {
        let color = Color::new(255, 0, 0);
        let serialized = serde_json::to_string(&color).unwrap();
        
        assert_eq!(serialized, "\"#FF0000\"");
        
        let color = serde_json::from_str::<Color>(&serialized).unwrap();
        
        assert_eq!(color, Color::new(255, 0, 0));
        
        let color = serde_json::from_str::<Color>("\"rgba(255,0,0,0.5)\"").unwrap();
        
        assert_eq!(color, Color::new(255, 0, 0));
    }
    
    #[test]
    fn test_all_serializers() {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Colors {
            #[serde(with = "hex")]
            hex: Color,
            #[serde(with = "rgb")]
            rgb: Color,
            #[serde(with = "rgba")]
            rgba: (Color, f32),
            #[serde(with = "hex_option")]
            hex_option: Option<Color>,
            #[serde(with = "rgb_option")]
            rgb_option: Option<Color>,
            #[serde(with = "rgba_option")]
            rgba_option: Option<(Color, f32)>,
        }
        
        let s = serde_json::to_string(&Colors {
            hex: Color::new(255, 0, 0),
            rgb: Color::new(255, 0, 0),
            rgba: (Color::new(255, 0, 0), 0.5),
            hex_option: Some(Color::new(255, 0, 0)),
            rgb_option: Some(Color::new(255, 0, 0)),
            rgba_option: Some((Color::new(255, 0, 0), 0.5)),
        }).unwrap();
        
        assert_eq!(s, "{\"hex\":\"#FF0000\",\"rgb\":\"rgb(255,0,0)\",\"rgba\":\"rgba(255,0,0,0.5)\",\"hex_option\":\"#FF0000\",\"rgb_option\":\"rgb(255,0,0)\",\"rgba_option\":\"rgba(255,0,0,0.5)\"}");
    }
}