//! Main UI rendering logic.
//!
//! This module handles the layout and drawing of the application's TUI interface.
//! It decomposes the screen into atomic regions (header, sidebar, viewport, status bar)
//! and delegates rendering to specialized helper functions.

use crate::app::{ActiveWidget, App, AppMode};
use crate::components::crop::render_crop_options;
use crate::theme::{STYLES, THEME};
use ansi_to_tui::IntoText;
use hdim_core::state::Tool;
use hdim_core::utils::file_name_from_path;
use hdim_render::view::View;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph},
};

/// Renders the complete application UI for the current frame.
///
/// This function is the entry point for the draw loop. It sets up the global layout
/// and coordinates the rendering of all sub-components.
///
/// # Arguments
///
/// * `frame` - The [Frame] buffer to draw into.
/// * `app` - The current [App] state.
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Base background
    frame.render_widget(Block::default().style(STYLES.base), area);

    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Navigation Bar
            Constraint::Min(0),    // Middle section
            Constraint::Length(1), // Bottom Status Line
        ])
        .split(area);

    render_top_nav(frame, app, global_layout[0]);
    render_main_section(frame, app, global_layout[1]);
    render_status_line(frame, app, global_layout[2]);

    if let AppMode::Saving = app.mode {
        render_save_as_popup(frame, app);
    }
}

/// Renders the top navigation bar containing the title and image filename.
fn render_top_nav(frame: &mut Frame, app: &App, area: Rect) {
    let image_name =
        file_name_from_path(&app.hdim_image.path).unwrap_or_else(|| "Unknown".to_string());

    let title = Line::from(vec![
        Span::styled(" HDIM ", STYLES.accent.add_modifier(Modifier::BOLD)),
        Span::styled("│ ", STYLES.border),
        Span::styled(image_name, Style::default().add_modifier(Modifier::BOLD)),
    ]);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(STYLES.border)
        .padding(Padding::horizontal(2));

    let paragraph = Paragraph::new(title)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Renders the primary workspace, split into the left sidebar, image viewport, and right sidebar.
fn render_main_section(frame: &mut Frame, app: &mut App, area: Rect) {
    let right_width = calculate_sidebar_width(app);

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(18),
            Constraint::Min(0),
            Constraint::Length(right_width),
        ])
        .split(area);

    render_left_sidebar(frame, app, main_layout[0]);
    render_image_window(frame, app, main_layout[1]);
    if right_width > 0 {
        render_right_sidebar(frame, app, main_layout[2]);
    }
}

/// Determines the width of the right sidebar based on the active tool or widget.
fn calculate_sidebar_width(app: &App) -> u16 {
    if app.selected_tool.is_some()
        || app.active_widget == ActiveWidget::Adjustments
        || app.mode == AppMode::ExifView
    {
        32
    } else {
        0
    }
}

/// Renders the left sidebar containing the list of available tools.
fn render_left_sidebar(frame: &mut Frame, app: &App, area: Rect) {
    let tools = vec![
        ("1", "Crop", Tool::Crop),
        ("2", "Metadata", Tool::Exif),
        ("3", "Export", Tool::Exif), // Using tool as placeholder
        ("4", "Adjust", Tool::Exif),
    ];

    let items: Vec<ListItem> = tools
        .into_iter()
        .map(|(key, name, _tool)| {
            let style = if (app.active_widget == ActiveWidget::Tools)
                || (name == "Adjust" && app.active_widget == ActiveWidget::Adjustments)
                || (name == "Metadata" && app.mode == AppMode::ExifView)
                || (name == "Crop" && app.selected_tool == Some(Tool::Crop))
            {
                STYLES.highlight
            } else {
                Style::default().fg(THEME.foreground).bg(THEME.background)
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", key), STYLES.accent),
                Span::raw(name),
            ]))
            .style(style)
        })
        .collect();

    let block = Block::default()
        .style(STYLES.base)
        .borders(Borders::RIGHT)
        .border_style(STYLES.border)
        .title(" TOOLS ")
        .title_style(STYLES.text_dim)
        .padding(Padding::uniform(1));

    frame.render_widget(List::new(items).block(block), area);
}

/// Renders the central image viewport using the [hdim_render] crate.
fn render_image_window(frame: &mut Frame, app: &mut App, area: Rect) {
    let (source_width, source_height) =
        app.calculate_viewport(area.width as u32, area.height as u32);

    let view = View {
        source_x: app.source_pos.0,
        source_y: app.source_pos.1,
        source_width,
        source_height,
        target_width: area.width as u32,
        target_height: area.height as u32,
    };

    let image_text = match hdim_render::render(&app.cached_image, &view) {
        Ok(ansi_string) => ansi_string.into_text().unwrap_or_default(),
        Err(_) => "Error rendering image".into_text().unwrap(),
    };

    let block = Block::default()
        .style(STYLES.base)
        .padding(Padding::uniform(0))
        .border_style(STYLES.border);

    frame.render_widget(
        Paragraph::new(image_text).block(block).style(STYLES.base),
        area,
    );
}

/// Renders the context-sensitive right sidebar.
fn render_right_sidebar(frame: &mut Frame, app: &mut App, area: Rect) {
    if !app.show_right_toolbar {
        return;
    }

    let block = Block::default()
        .style(STYLES.base)
        .borders(Borders::LEFT)
        .border_style(STYLES.border)
        .padding(Padding::uniform(1));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    match app.active_widget {
        ActiveWidget::Adjustments => render_adjustment_sidebar(frame, app, inner_area),
        ActiveWidget::RightToolbar | ActiveWidget::Main | ActiveWidget::Tools => {
            render_tool_sidebar(frame, app, inner_area)
        }
    }
}

/// Renders the adjustment sliders panel.
fn render_adjustment_sidebar(frame: &mut Frame, app: &mut App, area: Rect) {
    let is_editing = app.mode == AppMode::EditingAdjustmentValue;
    app.adjustment_panel
        .render(frame, area, is_editing, &app.adjustment_input);
}

/// Renders tool-specific options (e.g., crop ratios or EXIF data).
fn render_tool_sidebar(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.mode {
        AppMode::ExifView => render_exif_sidebar(frame, app, area),
        _ => {
            if let Some(Tool::Crop) = app.selected_tool {
                frame.render_widget(render_crop_options(app), area);
            } else {
                render_sidebar_placeholder(frame, area);
            }
        }
    }
}

/// Renders the EXIF metadata viewer widget.
fn render_exif_sidebar(frame: &mut Frame, app: &mut App, area: Rect) {
    if let Some(exif_view) = &mut app.exif_view {
        let mut list = exif_view.widget();
        list = list.highlight_style(STYLES.highlight).block(
            Block::default()
                .title(" METADATA ")
                .title_style(STYLES.text_dim),
        );
        frame.render_stateful_widget(list, area, &mut exif_view.state);
    }
}

/// Renders a placeholder message when no tool is active.
fn render_sidebar_placeholder(frame: &mut Frame, area: Rect) {
    let placeholder = Paragraph::new("Select a tool to begin editing")
        .style(STYLES.text_dim)
        .alignment(Alignment::Center)
        .block(Block::default().padding(Padding::top(area.height / 2)));
    frame.render_widget(placeholder, area);
}

/// Renders the bottom status line with current mode, hints, and zoom level.
fn render_status_line(frame: &mut Frame, app: &App, area: Rect) {
    let mode_str = match app.mode {
        AppMode::Normal => " NORMAL ",
        AppMode::EditingAdjustmentValue => " EDIT VALUE ",
        AppMode::EditingCropValue => " CROP ",
        AppMode::ExifView => " METADATA ",
        AppMode::Saving => " EXPORTING ",
    };

    let hint = match app.active_widget {
        ActiveWidget::Adjustments => "↑/↓ Select • ←/→ Adjust • Enter Manual • Esc Back",
        _ => "Arrows Pan • PgUp/Dn Zoom • 1-4 Tools • q Quit",
    };

    let status_line = Line::from(vec![
        Span::styled(mode_str, STYLES.inverted),
        Span::raw(" "),
        Span::styled(hint, STYLES.text_dim),
        Span::raw(" "),
        Span::styled(format!(" Zoom: {:.2}x ", 1.0 / app.zoom), STYLES.text_dim),
    ]);

    frame.render_widget(
        Paragraph::new(status_line).style(Style::default().bg(THEME.surface)),
        area,
    );
}

/// Renders the "Save As" modal dialog.
fn render_save_as_popup(frame: &mut Frame, app: &App) {
    let size = frame.area();
    let popup_area = Rect::new(
        size.width.saturating_sub(60) / 2,
        size.height.saturating_sub(12) / 2,
        60,
        12,
    );

    frame.render_widget(Clear, popup_area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(STYLES.border_active)
        .title(" EXPORT IMAGE ")
        .title_alignment(Alignment::Center);

    frame.render_widget(&app.save_as, block.inner(popup_area));
    frame.render_widget(block, popup_area);
}
