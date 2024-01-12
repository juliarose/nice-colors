//! Serializers for the `Color` type.

use crate::Color;
use crate::html::from_html_color_name;
use serde::de;
use std::fmt;

/// Deserializes from hexademical and rgb color strings.
struct ColorVisitor;

impl<'de> de::Visitor<'de> for ColorVisitor {
    type Value = Color;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hexadecimal or rgba color color string")
    }
    
    /// Deserializes from a color string.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.starts_with("rgb") {
            return Self::Value::from_rgb(v).ok_or(serde::de::Error::custom("Not a valid rgb color string."));
        }
        
        match v.parse::<Self::Value>() {
            Ok(value) => Ok(value),
            Err(error) => if let Some(color) = from_html_color_name(v) {
                Ok(color)
            } else {
                Err(serde::de::Error::custom(error))
            },
        }
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
        d.deserialize_str(ColorVisitor).map(Some)
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
    type Value = (Color, Option<f32>);
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hexadecimal color string or none")
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
        
        match v.parse::<Color>() {
            Ok(value) => Ok((value, None)),
            Err(error) => if let Some(color) = from_html_color_name(v) {
                Ok((color, None))
            } else {
                Err(serde::de::Error::custom(error))
            },
        }
    }
}

/// Deserializes from optional hexademical and rgb color strings with alpha.
struct OptionColorAlphaVisitor;

impl<'de> de::Visitor<'de> for OptionColorAlphaVisitor {
    type Value = Option<(Color, Option<f32>)>;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hexadecimal or rgba color string or none")
    }
    
    /// Deserializes from a color string.
    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(ColorAlphaVisitor).map(Some)
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
    pub fn serialize<T, S>(value: &Color, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn serialize<T, S>(value: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn serialize<T, S>(value: &Color, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn serialize<T, S>(value: &Option<Color>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn serialize<T, S>(value: &(Color, Option<f32>), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(amount) = value.1 {
            serializer.collect_str(&format!("{}", value.0.to_rgba(amount)))
        } else {
            serializer.collect_str(&format!("{}", value.0.to_rgb()))
        }
    }
    
    /// Deserializes a color from an rgba string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<(Color, Option<f32>), D::Error>
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
    use crate::Color;
    use serde::{Serializer, Deserializer};
    
    /// Serializes a color to an rgba string.
    pub fn serialize<T, S>(value: &Option<(Color, Option<f32>)>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            if let Some(amount) = value.1 {
                serializer.collect_str(&format!("{}", value.0.to_rgba(amount)))
            } else {
                serializer.collect_str(&format!("{}", value.0.to_rgb()))
            }
        } else {
            serializer.serialize_none()
        }
    }
    
    /// Deserializes a color from an rgba string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<(Color, Option<f32>)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionColorAlphaVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;
    // use serde::{Serialize, Deserialize};
    
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
}