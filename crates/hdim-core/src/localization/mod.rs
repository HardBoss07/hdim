use serde::{Deserialize, Serialize};

pub mod de;
pub mod en;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Localization {
    pub tools: Tools,
    pub status: Status,
    pub common: Common,
    pub adjustments: Adjustments,
    pub exif: Exif,
    pub transform: Transform,
    pub export: Export,
    pub confirm_quit: ConfirmQuit,
    pub settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub title: String,
    pub language: String,
    pub theme: String,
    pub strip_exif: String,
    pub save: String,
    pub cancel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tools {
    pub title: String,
    pub transform: String,
    pub metadata: String,
    pub export: String,
    pub adjust: String,
    pub placeholder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub normal: String,
    pub edit_value: String,
    pub transform: String,
    pub metadata: String,
    pub exporting: String,
    pub confirm_quit: String,
    pub hint_adjust: String,
    pub hint_normal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Common {
    pub unknown: String,
    pub error_rendering: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adjustments {
    pub title: String,
    pub brightness: String,
    pub contrast: String,
    pub exposure: String,
    pub fade: String,
    pub grain: String,
    pub hue: String,
    pub noise: String,
    pub saturation: String,
    pub vibrance: String,
    pub warmth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exif {
    pub title: String,
    pub general: String,
    pub date_time: String,
    pub camera: String,
    pub make: String,
    pub model: String,
    pub software: String,
    pub exposure: String,
    pub exposure_time: String,
    pub f_number: String,
    pub iso: String,
    pub lens: String,
    pub focal_length: String,
    pub f_number_range: String,
    pub image: String,
    pub width: String,
    pub height: String,
    pub gps: String,
    pub latitude: String,
    pub longitude: String,
    pub altitude: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub title: String,
    pub left: String,
    pub right: String,
    pub top: String,
    pub bottom: String,
    pub rotate_left: String,
    pub rotate_right: String,
    pub flip_horizontal: String,
    pub flip_vertical: String,
    pub from_viewport: String,
    pub apply: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub title: String,
    pub select_format: String,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmQuit {
    pub title: String,
    pub message: String,
    pub question: String,
    pub yes: String,
    pub no: String,
}
