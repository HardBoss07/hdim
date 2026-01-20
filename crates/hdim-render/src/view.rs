/// Defines the mapping between a rectangular area of the source image
/// and the target rendering area in the terminal.
pub struct View {
    /// The top-left X coordinate of the view on the source image (in pixels).
    pub source_x: u32,
    /// The top-left Y coordinate of the view on the source image (in pixels).
    pub source_y: u32,
    /// The width of the view on the source image (in pixels).
    pub source_width: u32,
    /// The height of the view on the source image (in pixels).
    pub source_height: u32,
    /// The width of the target render area (in terminal columns).
    pub target_width: u32,
    /// The height of the target render area (in terminal rows).
    pub target_height: u32,
}
