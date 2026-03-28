use hdim_core::exif::ExifData;
use hdim_core::localization::Exif as ExifLocalization;
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};

pub struct ExifView {
    pub exif_data: ExifData,
    pub state: ListState,
}

impl ExifView {
    pub fn new(exif_data: &ExifData) -> Self {
        Self {
            exif_data: exif_data.clone(),
            state: ListState::default(),
        }
    }

    pub fn widget<'a>(&self, loc: &'a ExifLocalization) -> List<'a> {
        let mut items = Vec::new();

        if self.exif_data.is_empty() {
            items.push(
                ListItem::new(loc.no_data.clone()).style(
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::ITALIC),
                ),
            );
            return List::new(items);
        }

        // General Info
        items.push(
            ListItem::new(loc.general.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
        );
        if let Some(datetime) = &self.exif_data.datetime
            && let Some(original) = &datetime.original
        {
            items.push(ListItem::new(format!("{}{}", loc.date_time, original)));
        }

        // Camera Info
        items.push(
            ListItem::new(loc.camera.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
        );
        if let Some(camera) = &self.exif_data.camera {
            if let Some(make) = &camera.make {
                items.push(ListItem::new(format!("{}{}", loc.make, make)));
            }
            if let Some(model) = &camera.model {
                items.push(ListItem::new(format!("{}{}", loc.model, model)));
            }
            if let Some(software) = &camera.software {
                items.push(ListItem::new(format!("{}{}", loc.software, software)));
            }
        }

        // Exposure Info
        items.push(
            ListItem::new(loc.exposure.clone())
                .style(Style::default().add_modifier(Modifier::BOLD)),
        );
        if let Some(exposure) = &self.exif_data.exposure {
            if let Some(exposure_time) = &exposure.exposure_time {
                items.push(ListItem::new(format!(
                    "{}{}",
                    loc.exposure_time, exposure_time
                )));
            }
            if let Some(f_number) = &exposure.f_number {
                items.push(ListItem::new(format!("{}{}", loc.f_number, f_number)));
            }
            if let Some(iso) = &exposure.iso {
                items.push(ListItem::new(format!("{}{}", loc.iso, iso)));
            }
        }

        // Lens Info
        items.push(
            ListItem::new(loc.lens.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
        );
        if let Some(lens) = &self.exif_data.lens {
            if let Some(focal_length) = &lens.focal_length {
                items.push(ListItem::new(format!(
                    "{}{}",
                    loc.focal_length, focal_length
                )));
            }
            if let Some(f_number_range) = &lens.f_number_range {
                items.push(ListItem::new(format!(
                    "{}{}",
                    loc.f_number_range, f_number_range
                )));
            }
        }

        // Image Info
        items.push(
            ListItem::new(loc.image.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
        );
        if let Some(image) = &self.exif_data.image {
            if let Some(width) = image.width {
                items.push(ListItem::new(format!("{}{}", loc.width, width)));
            }
            if let Some(height) = image.height {
                items.push(ListItem::new(format!("{}{}", loc.height, height)));
            }
        }

        // GPS Info
        items.push(
            ListItem::new(loc.gps.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
        );
        if let Some(gps) = &self.exif_data.gps {
            if let Some(latitude) = &gps.latitude {
                items.push(ListItem::new(format!("{}{}", loc.latitude, latitude)));
            }
            if let Some(longitude) = &gps.longitude {
                items.push(ListItem::new(format!("{}{}", loc.longitude, longitude)));
            }
            if let Some(altitude) = &gps.altitude {
                items.push(ListItem::new(format!("{}{}", loc.altitude, altitude)));
            }
        }

        List::new(items)
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= 20 {
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
                    20
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
