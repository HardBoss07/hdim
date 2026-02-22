use hdim_core::exif::ExifData;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, List, ListItem, ListState},
};

pub struct ExifView {
    pub state: ListState,
    items: Vec<ListItem<'static>>,
}

impl ExifView {
    pub fn new(exif_data: &ExifData) -> Self {
        let mut items = Vec::new();
        items.push(ListItem::new("General:").style(Style::default().add_modifier(Modifier::BOLD)));
        if let Some(datetime) = &exif_data.datetime {
            if let Some(original) = &datetime.original {
                items.push(ListItem::new(format!("  Date Time: {}", original)));
            }
        }

        if let Some(camera) = &exif_data.camera {
            items.push(
                ListItem::new("Camera:").style(Style::default().add_modifier(Modifier::BOLD)),
            );
            if let Some(make) = &camera.make {
                items.push(ListItem::new(format!("  Make: {}", make)));
            }
            if let Some(model) = &camera.model {
                items.push(ListItem::new(format!("  Model: {}", model)));
            }
            if let Some(software) = &camera.software {
                items.push(ListItem::new(format!("  Software: {}", software)));
            }
        }

        if let Some(exposure) = &exif_data.exposure {
            items.push(
                ListItem::new("Exposure:").style(Style::default().add_modifier(Modifier::BOLD)),
            );
            if let Some(exposure_time) = exposure.exposure_time {
                items.push(ListItem::new(format!("  Exposure Time: {}", exposure_time)));
            }
            if let Some(f_number) = exposure.f_number {
                items.push(ListItem::new(format!("  F Number: {}", f_number)));
            }
            if let Some(iso) = exposure.iso {
                items.push(ListItem::new(format!("  ISO: {}", iso)));
            }
        }

        if let Some(lens) = &exif_data.lens {
            items.push(ListItem::new("Lens:").style(Style::default().add_modifier(Modifier::BOLD)));
            if let Some(focal_length) = &lens.focal_length {
                items.push(ListItem::new(format!("  Focal Length: {}", focal_length)));
            }
            if let Some(f_number_range) = &lens.f_number_range {
                items.push(ListItem::new(format!(
                    "  F Number Range: {}",
                    f_number_range
                )));
            }
        }

        if let Some(image) = &exif_data.image {
            items
                .push(ListItem::new("Image:").style(Style::default().add_modifier(Modifier::BOLD)));
            if let Some(width) = &image.width {
                items.push(ListItem::new(format!("  Width: {}", width)));
            }
            if let Some(height) = &image.height {
                items.push(ListItem::new(format!("  Height: {}", height)));
            }
        }

        if let Some(gps) = &exif_data.gps {
            items.push(ListItem::new("GPS:").style(Style::default().add_modifier(Modifier::BOLD)));
            if let Some(latitude) = &gps.latitude {
                items.push(ListItem::new(format!("  Latitude: {}", latitude)));
            }
            if let Some(longitude) = &gps.longitude {
                items.push(ListItem::new(format!("  Longitude: {}", longitude)));
            }
            if let Some(altitude) = &gps.altitude {
                items.push(ListItem::new(format!("  Altitude: {}", altitude)));
            }
        }

        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn widget(&self) -> List<'static> {
        List::new(self.items.clone())
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("EXIF Data"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    }
}
