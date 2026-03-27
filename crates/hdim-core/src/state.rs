//! Application state management.
//!
//! This module defines the core state structures used to track the active tool
//! and tool-specific configuration (like crop boundaries).

/// Represents the currently active tool in the editor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    /// The transform tool, allowing rotation, flipping, and cropping.
    Transform,
    /// The EXIF metadata viewer.
    Exif,
}

/// Represents the state of the transform tool operation.
#[derive(Clone, Copy, Debug)]
pub struct TransformState {
    /// Pixels to remove from the left edge.
    pub left: u32,
    /// Pixels to remove from the right edge.
    pub right: u32,
    /// Pixels to remove from the top edge.
    pub top: u32,
    /// Pixels to remove from the bottom edge.
    pub bottom: u32,
    /// Rotation in degrees (0, 90, 180, 270).
    pub rotation: i32,
    /// Whether the image is flipped horizontally.
    pub flip_horizontal: bool,
    /// Whether the image is flipped vertically.
    pub flip_vertical: bool,
}

impl Default for TransformState {
    fn default() -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            rotation: 0,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
}
