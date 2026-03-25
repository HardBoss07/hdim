//! Image adjustment algorithms.
//!
//! This module contains the implementation of various image filters and
//! adjustments, such as brightness, contrast, exposure, and color-specific
//! transformations. Each sub-module provides an atomic function to apply
//! its specific transformation to a [image::DynamicImage].

pub mod brightness;
pub mod contrast;
pub mod exposure;
pub mod fade;
pub mod grain;
pub mod hue;
pub mod noise;
pub mod saturation;
pub mod vibrance;
pub mod warmth;
