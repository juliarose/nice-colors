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

pub(crate) mod helpers;
pub(crate) mod color;
pub(crate) mod hsl_color;
mod parse;

pub use color::{Color, ColorWithAlpha};
pub use hsl_color::HSLColor;