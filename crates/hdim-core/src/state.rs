//! Application state management.
//!
//! This module defines the core state structures used to track the active tool
//! and tool-specific configuration (like crop boundaries).

/// Represents the currently active tool in the editor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    /// The crop tool, allowing the user to select a rectangular region.
    Crop,
    /// The EXIF metadata viewer.
    Exif,
}

/// Represents the state of the crop tool selection.
///
/// Values indicate the number of pixels to crop from each edge.
#[derive(Clone, Copy, Debug)]
pub struct CropState {
    /// Pixels to remove from the left edge.
    pub left: u32,
    /// Pixels to remove from the right edge.
    pub right: u32,
    /// Pixels to remove from the top edge.
    pub top: u32,
    /// Pixels to remove from the bottom edge.
    pub bottom: u32,
}

impl Default for CropState {
    fn default() -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }
}
