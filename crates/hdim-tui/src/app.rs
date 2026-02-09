use crate::components::exif_view::ExifView;
use crate::components::save_as::SaveAs;
use color_eyre::eyre::{Ok, Result};
use hdim_core::{
    HdimImage,
    exif::ExifData,
    state::{CropState, Tool},
};
use std::{
    fs::File,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveWidget {
    Main,
    Tools,
    RightToolbar,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppMode {
    Normal,
    EditingCropValue,
    ExifView,
    Saving,
}

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
    // The currently selected tool
    pub selected_tool: Option<Tool>,
    // The currently active widget
    pub active_widget: ActiveWidget,
    // The state of the crop tool
    pub crop_state: CropState,
    // The current application mode
    pub mode: AppMode,
    // The index of the selected crop option
    pub selected_crop_option_index: usize,
    // The input string for crop values
    pub crop_input: String,
    // The EXIF data of the image
    pub exif_data: Option<ExifData>,
    // The state of the EXIF view
    pub exif_view: Option<ExifView>,
    // Whether to show the right toolbar
    pub show_right_toolbar: bool,
    // The state of the save as component
    pub save_as: SaveAs,
}

impl App {
    pub fn new(hdim_image: HdimImage, initial_zoom: f32) -> Result<Self> {
        let mut file = File::open(hdim_image.path.clone())?;
        let exif_data = ExifData::get_exif_data(&mut file).ok();
        let exif_view = exif_data.as_ref().map(ExifView::new);

        Ok(Self {
            hdim_image,
            source_pos: (0, 0),
            zoom: initial_zoom,
            last_input_time: Instant::now(),
            input_delay: Duration::from_millis(50), // Reduced for snappier input
            selected_tool: None,
            active_widget: ActiveWidget::Main,
            crop_state: CropState::default(),
            mode: AppMode::Normal,
            selected_crop_option_index: 0,
            crop_input: String::new(),
            exif_data,
            exif_view,
            show_right_toolbar: true,
            save_as: SaveAs::new(),
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
    pub fn scroll(&mut self, delta_x: i32, delta_y: i32) {
        self.source_pos.0 = self.source_pos.0.saturating_add_signed(delta_x);
        self.source_pos.1 = self.source_pos.1.saturating_add_signed(delta_y);
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

    /// Calculates the viewport dimensions and clamps the source position.
    /// Returns the calculated source width and height in pixels.
    pub fn calculate_viewport(&mut self, target_width: u32, target_height: u32) -> (u32, u32) {
        let source_width = (target_width as f32 * self.zoom).round() as u32;
        let source_height = (target_height as f32 * self.zoom * 2.0).round() as u32;

        let image_width = self.hdim_image.width;
        let image_height = self.hdim_image.height;

        self.source_pos.0 = self
            .source_pos
            .0
            .min(image_width.saturating_sub(source_width));
        self.source_pos.1 = self
            .source_pos
            .1
            .min(image_height.saturating_sub(source_height));

        (source_width, source_height)
    }
}
