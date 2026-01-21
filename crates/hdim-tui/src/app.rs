use color_eyre::eyre::{Ok, Result};
use hdim_core::HdimImage;
use std::time::{Duration, Instant};

/// Application state
pub struct App {
    /// We store the wrapper HdimImage so we can re-render it and access metadata
    pub hdim_image: HdimImage,
    /// The top-left corner of the viewport on the source image (x, y) in pixels.
    pub source_pos: (u32, u32),
    /// Zoom level. Represents `source_pixels / terminal_characters`.
    /// A smaller value is more zoomed in.
    pub zoom: f32,
    /// Track the last time an input was processed to prevent double-triggering
    pub last_input_time: Instant,
    /// Minimum time between processing consecutive inputs
    pub input_delay: Duration,
}

impl App {
    pub fn new(hdim_image: HdimImage, initial_zoom: f32) -> Result<Self> {
        Ok(Self {
            hdim_image,
            source_pos: (0, 0),
            zoom: initial_zoom,
            last_input_time: Instant::now(),
            input_delay: Duration::from_millis(50), // Reduced for snappier input
        })
    }

    /// Adjusts the zoom level.
    pub fn zoom(&mut self, factor: f32) {
        self.zoom *= factor;
        // Clamp zoom to a reasonable range
        if self.zoom < 0.01 {
            self.zoom = 0.01;
        }
        self.clamp_source_pos();
    }

    /// Moves the viewport on the source image.
    pub fn scroll(&mut self, dx: i32, dy: i32) {
        self.source_pos.0 = self.source_pos.0.saturating_add_signed(dx);
        self.source_pos.1 = self.source_pos.1.saturating_add_signed(dy);
        self.clamp_source_pos();
    }

    // Prevents the viewport from going out of bounds of the source image.
    pub fn clamp_source_pos(&mut self) {
        let image_width = self.hdim_image.width;
        let image_height = self.hdim_image.height;
        if self.source_pos.0 > image_width {
            self.source_pos.0 = image_width;
        }
        if self.source_pos.1 > image_height {
            self.source_pos.1 = image_height;
        }
    }
}
