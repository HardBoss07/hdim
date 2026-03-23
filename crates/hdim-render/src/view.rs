/// Defines the mapping between a rectangular area of the source image
/// and the target rendering area in the terminal.
#[derive(Debug, Clone, Copy)]
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

impl View {
    /// Calculates the scaling ratios between source and target dimensions.
    /// Returns (x_ratio, y_ratio).
    pub fn calculate_scaling(&self) -> (f32, f32) {
        let x_ratio = self.source_width as f32 / self.target_width as f32;
        let y_ratio = self.source_height as f32 / self.target_height as f32;
        (x_ratio, y_ratio)
    }

    /// Maps a terminal cell coordinate (x, y) to the corresponding source image coordinates.
    /// Returns (source_x, source_y_top, source_y_bottom).
    pub fn map_to_source(&self, x: u32, y: u32, x_ratio: f32, y_ratio: f32) -> (u32, u32, u32) {
        let source_pixel_x = self.source_x + (x as f32 * x_ratio) as u32;
        let source_pixel_y_top = self.source_y + (y as f32 * y_ratio) as u32;
        let source_pixel_y_bottom = source_pixel_y_top + (y_ratio / 2.0).round().max(1.0) as u32;
        (source_pixel_x, source_pixel_y_top, source_pixel_y_bottom)
    }

    /// Calculates the block dimensions for averaging colors.
    pub fn calculate_block_size(&self, x_ratio: f32, y_ratio: f32) -> (u32, u32) {
        let block_width = x_ratio.round().max(1.0) as u32;
        let block_height = (y_ratio / 2.0).round().max(1.0) as u32;
        (block_width, block_height)
    }
}
