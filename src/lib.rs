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
mod parse;
mod color;

pub(crate) type Value = u8;
pub(crate) type Alpha = f32;
pub(crate) type DecimalValue = u32;

pub(crate) const SLICE_LENGTH: usize = 3;

pub use color::{Color, ColorWithAlpha};
