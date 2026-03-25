pub mod colors;
pub mod style;

pub use colors::{Palette, get_palette};
pub use style::ThemeStyles;

pub const SLIDER_TRACK: &str = "░";
pub const SLIDER_FILL: &str = "█";
pub const SLIDER_HANDLE: &str = "⬢"; // Sleek hexagonal handle
