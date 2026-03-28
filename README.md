# HDIM - High Definition Image Manipulator

**HDIM** is a modular, component-based Terminal User Interface (TUI) application for high-definition image manipulation. It brings a modern image editing experience directly to your terminal, similar to standard mobile editing tools but optimized for the command line.

## Key Features

- **High-Definition Rendering:** Uses half-block character techniques to render images with incredible detail in the terminal.
- **Non-Destructive Editing:** Full Undo/Redo history stack for all your adjustments.
- **Transform Tools:** Rotate, flip, and crop images (supporting both absolute pixels and relative percentages).
- **Advanced Adjustments:** Fine-tune your images with sliders for:
    - Saturation & Vibrance
    - Exposure, Brightness & Contrast
    - Warmth & Hue
    - Fade (Matte effect)
    - Film Grain & RGB Noise
- **Metadata Viewer:** Comprehensive EXIF data display.
- **Save & Export:** Save your changes to various formats (PNG, JPEG, GIF, BMP) with optional EXIF stripping for privacy.
- **Localization:** Supports both English and German languages.
- **Cross-Platform:** Works on Windows, macOS, and Linux.

## Requirements

To use HDIM, your terminal must support **TrueColor (24-bit)**. Verified terminals include:
- VS Code Integrated Terminal
- Windows Terminal (PowerShell/CMD)
- iTerm2 / Apple Terminal
- Alacritty / Kitty / GNOME Terminal

## Installation

```bash
# Clone the repository
git clone https://github.com/mttvll/hdim
cd hdim

# Install the TUI application
cargo install --path crates/hdim-tui
```

## Quick Start

Run the application by providing a path to an image:

```bash
hdim path/to/your/image.jpg
```

### Keybindings

- **1-5:** Switch between tools (Transform, Metadata, Export, Adjust, Settings).
- **Arrows:** Pan the image (in Viewport mode) or navigate menus.
- **PageUp / PageDown:** Zoom in and out.
- **Ctrl + z / Ctrl + y:** Undo and Redo.
- **Ctrl + s:** Open the Save/Export dialog.
- **Ctrl + q:** Quit the application (with a warning if there are unsaved changes).
- **Esc:** Return to the main viewport or cancel current mode.

## For Developers

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on coding standards, structural updates, and our mandatory workflow.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
