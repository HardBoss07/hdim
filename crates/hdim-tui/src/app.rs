//! Main application logic and state management.
//!
//! This module defines the [App] struct, which holds the global state of the application,
//! including the loaded image, current adjustments, UI mode, and input handling.

use crate::components::adjustment_panel::AdjustmentPanel;
use crate::components::exif_view::ExifView;
use crate::components::save_as::SaveAs;
use crate::components::settings::SettingsView;
use crate::config::Config;
use crate::theme::{Palette, ThemeStyles, get_palette};
use color_eyre::eyre::{Ok, Result};
use hdim_core::{
    HdimImage,
    exif::ExifData,
    localization::Localization,
    state::{Tool, TransformState},
};
use image::DynamicImage;
use std::{
    fs::File,
    time::{Duration, Instant},
};

/// Represents the currently focused UI region.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveWidget {
    /// The central image viewport.
    Main,
    /// The left-hand tool selection sidebar.
    Tools,
    /// The right-hand adjustment slider panel.
    Adjustments,
    /// The generic right toolbar container.
    RightToolbar,
}

/// Represents the current operational mode of the application.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppMode {
    /// Standard navigation mode.
    Normal,
    /// User is inputting transform values.
    EditingTransformValue,
    /// User is modifying a specific adjustment slider.
    EditingAdjustmentValue,
    /// Viewing EXIF metadata.
    ExifView,
    /// Exporting the image.
    Saving,
    /// Confirming quit with unsaved changes.
    ConfirmQuit,
    /// Confirming cancel transform with unapplied changes.
    ConfirmTransformCancel,
    /// Settings screen.
    Settings,
}

/// The global application state.
///
/// `App` orchestrates the interaction between the core image logic ([HdimImage])
/// and the TUI rendering layer. It manages the event loop state, input buffers,
/// and widget coordination.
pub struct App {
    /// The core image data wrapper, preserving original state and metadata.
    pub hdim_image: HdimImage,
    /// A cached version of the image with all current adjustments applied.
    /// Used for rendering to avoid re-processing on every frame.
    pub cached_image: DynamicImage,
    /// The top-left corner of the viewport on the source image (x, y) in pixels.
    pub source_pos: (u32, u32),
    /// Zoom level where 1.0 is 1:1 pixel mapping.
    /// Smaller values represent "zooming out" (seeing more of the image).
    pub zoom: f32,
    /// Timestamp of the last processed input event, used for debouncing.
    pub last_input_time: Instant,
    /// Minimum duration between processing consecutive input events.
    pub input_delay: Duration,
    /// The tool currently selected by the user (e.g., Transform).
    pub selected_tool: Option<Tool>,
    /// The UI widget currently receiving input focus.
    pub active_widget: ActiveWidget,
    /// State specific to the transform tool operation.
    pub transform_state: TransformState,
    /// The current interaction mode of the application.
    pub mode: AppMode,
    /// Index of the currently selected option in the transform menu.
    pub selected_transform_option_index: usize,
    /// Buffer for manual transform value input.
    pub transform_input: String,
    /// Buffer for manual adjustment value input.
    pub adjustment_input: String,
    /// Parsed EXIF metadata, if available.
    pub exif_data: Option<ExifData>,
    /// The view component for displaying EXIF data.
    pub exif_view: Option<ExifView>,
    /// Flag to toggle the visibility of the right sidebar.
    pub show_right_toolbar: bool,
    /// The "Save As" dialog component state.
    pub save_as: SaveAs,
    /// The adjustment sliders panel component state.
    pub adjustment_panel: AdjustmentPanel,
    /// The index in history that corresponds to the last saved state.
    pub last_saved_index: usize,
    /// Last target viewport size (width, height) in columns/rows.
    pub last_viewport_size: (u16, u16),
    /// Persistent configuration.
    pub config: Config,
    /// Localized strings.
    pub localization: Localization,
    /// Settings view state.
    pub settings_view: Option<SettingsView>,
    /// Current color palette.
    pub palette: Palette,
    /// Current UI styles.
    pub styles: ThemeStyles,
    /// Whether there are transform changes that haven't been applied yet.
    pub has_unapplied_transform: bool,
}

impl App {
    /// Creates a new `App` instance with the loaded image.
    ///
    /// # Errors
    ///
    /// Returns an error if the image file cannot be re-opened to parse EXIF data.
    pub fn new(hdim_image: HdimImage, initial_zoom: f32) -> Result<Self> {
        let mut file = File::open(hdim_image.path.clone())?;
        let exif_data = ExifData::get_exif_data(&mut file).ok();
        let exif_view = exif_data.as_ref().map(ExifView::new);

        let config = Config::load();
        let localization = config.language.get_localization();
        let palette = get_palette(&config.theme);
        let styles = ThemeStyles::new(&palette);

        let adjustment_panel =
            AdjustmentPanel::new(hdim_image.adjustments, &localization.adjustments);
        let cached_image = hdim_image.apply_adjustments();

        Ok(Self {
            hdim_image,
            cached_image,
            source_pos: (0, 0),
            zoom: initial_zoom,
            last_input_time: Instant::now(),
            input_delay: Duration::from_millis(50), // Reduced for snappier input
            selected_tool: None,
            active_widget: ActiveWidget::Main,
            transform_state: TransformState::default(),
            mode: AppMode::Normal,
            selected_transform_option_index: 0,
            transform_input: String::new(),
            adjustment_input: String::new(),
            exif_data,
            exif_view,
            show_right_toolbar: true,
            save_as: SaveAs::new(),
            adjustment_panel,
            last_saved_index: 0,
            last_viewport_size: (0, 0),
            config,
            localization,
            settings_view: None,
            palette,
            styles,
            has_unapplied_transform: false,
        })
    }

    /// Refreshes the localization strings based on current config.
    pub fn refresh_localization(&mut self) {
        self.localization = self.config.language.get_localization();
        self.palette = get_palette(&self.config.theme);
        self.styles = ThemeStyles::new(&self.palette);
        // Also need to refresh the adjustment panel because it stores labels in Sliders
        self.adjustment_panel =
            AdjustmentPanel::new(self.hdim_image.adjustments, &self.localization.adjustments);
    }

    /// Adjusts the zoom level by a multiplication factor.
    ///
    /// Clamps the minimum zoom to prevent invalid states.
    ///
    /// # Arguments
    ///
    /// * `factor` - The multiplier to apply to the current zoom level.
    pub fn zoom(&mut self, factor: f32) {
        self.zoom *= factor;
        // Clamp zoom to a reasonable range
        // Hard limit of 2x (0.5) to prevent rendering breakdowns
        if self.zoom < 0.5 {
            self.zoom = 0.5;
        }
        self.clamp_source_pos();
    }

    /// Checks if there are unsaved changes in the history.
    pub fn has_unsaved_changes(&self) -> bool {
        self.hdim_image.history.current_index() != self.last_saved_index
    }

    /// Marks the current state as saved.
    pub fn mark_saved(&mut self) {
        self.last_saved_index = self.hdim_image.history.current_index();
    }

    /// Pans the viewport across the source image.
    ///
    /// # Arguments
    ///
    /// * `delta_x` - Horizontal pixels to move (positive = right).
    /// * `delta_y` - Vertical pixels to move (positive = down).
    pub fn scroll(&mut self, delta_x: i32, delta_y: i32) {
        self.source_pos.0 = self.source_pos.0.saturating_add_signed(delta_x);
        self.source_pos.1 = self.source_pos.1.saturating_add_signed(delta_y);
        self.clamp_source_pos();
    }

    /// Ensures the viewport position remains within the bounds of the image.
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

    /// Applies the current adjustments from the panel to the image.
    ///
    /// This updates the `cached_image` used for rendering.
    pub fn update_adjustments(&mut self) {
        self.hdim_image.adjustments = self.adjustment_panel.get_adjustments();
        self.cached_image = self.hdim_image.apply_adjustments();
    }

    /// Calculates the source viewport dimensions based on the target terminal size and zoom.
    ///
    /// Also clamps the source position to ensure the viewport is valid.
    ///
    /// # Arguments
    ///
    /// * `target_width` - The width of the available terminal area in columns.
    /// * `target_height` - The height of the available terminal area in rows.
    ///
    /// # Returns
    ///
    /// A tuple `(source_width, source_height)` in image pixels.
    pub fn calculate_viewport(&mut self, target_width: u32, target_height: u32) -> (u32, u32) {
        self.last_viewport_size = (target_width as u16, target_height as u16);
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

    /// Sets the transform state to crop exactly what is currently visible in the viewport.
    pub fn crop_from_viewport(&mut self) {
        let (source_width, source_height) = self.calculate_viewport(
            self.last_viewport_size.0 as u32,
            self.last_viewport_size.1 as u32,
        );

        self.transform_state.left = self.source_pos.0;
        self.transform_state.top = self.source_pos.1;
        self.transform_state.right = self
            .hdim_image
            .width
            .saturating_sub(self.source_pos.0 + source_width);
        self.transform_state.bottom = self
            .hdim_image
            .height
            .saturating_sub(self.source_pos.1 + source_height);
    }
}
