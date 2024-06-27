//! # nice-colors
//! 
//! Zero-dependency module (support for serde is optional) for working with colors. The aim is to 
//! provide a wrapper for RGB color values in a way that does not compromise performance. Colors 
//! are packed as three `u8` values to provide as lean of a memory footprint as possible. While 
//! there are also some helpers for working with alpha values, they are not a main feature of this 
//! module.
//! 
//! ## Features
//! - Color manipulation (lighten, darken, saturate, desaturate, etc.)
//! - CSS color parsing (RGB, RGBA, HSL, HSLA). Not all CSS color formats are supported but 
//! provides enough for most use cases.
//! - Color serialization to and from CSS color strings.

#![warn(missing_docs)]

#[cfg(feature = "serde")]
pub mod serializers;
pub mod html;

mod helpers;
mod color;
mod hsl_color;
mod parse;

pub use color::{Color, ColorWithAlpha};
pub use hsl_color::HSLColor;