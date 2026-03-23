# hdim - High Definition Image Manipulator

## General

- Keep this `GEMINI.md` updated.
- Use `project-structure .` for project tree.
- Do **not** run the program.
- Use `cargo check` for syntax / compiler errors.
- Do not use emojis in the responses, especially the code you're writing.
- **Formatting:** Always run `cargo fmt` after writing code.
- You must **check off** the checkboxes that are done.

The project is a Rust workspace composed of three crates located in `crates/`: `hdim-core`, `hdim-render`, and `hdim-tui`.

## Goal

The goal is to build a modular, component-based TUI application for high-definition image manipulation, similar to standard mobile editing tools but in the terminal.

## Architecture & File Structure

The workspace is defined in the root `Cargo.toml`.

### 1. hdim-core (`crates/hdim-core`)

**Purpose:** Pure data manipulation, EXIF parsing, and state management.

- **Modules:**
- `src/exif/`: Comprehensive EXIF data handling (`camera.rs`, `gps.rs`, `exposure.rs`, etc.) using `kamadak-exif`.
- `src/state.rs`: Manages the current editing session state (`CropState`, `Tool`).
- `src/lib.rs`: Entry point defining `HdimImage`.

- **Testing:** Unit tests (e.g., `tests/resizing.rs`).

### 2. hdim-render (`crates/hdim-render`)

**Purpose:** Translating image buffers into terminal-renderable cells.

- **Modules:**
- `src/pixel.rs`: Logic for pixel data handling and color conversion.
- `src/view.rs`: Viewport management for rendering specific sections of an image.

- **Testing:** Heavily relies on **Snapshot Testing** using `insta`.
- `tests/images/`: Test assets (4k.jpg, WindowsXP.png).
- `tests/snapshots/`: Stored snap files verifying render output consistency.

### 3. hdim-tui (`crates/hdim-tui`)

**Purpose:** The executable CLI/TUI application.

- **Modules:**
- `src/components/`: Modular UI widgets.
- `crop.rs`: Crop tool logic.
- `exif_view.rs`: EXIF data display widget.

- `src/app.rs`: Application state (`AppMode`, `ActiveWidget`), zoom/scroll logic, and lifecycle management.
- `src/events.rs`: Input event handling loop.
- `src/ui.rs`: Main drawing logic using `ratatui`.
- `src/main.rs`: Entry point, manual argument parsing (image path).

## Data Flow

1. **Load:** `hdim-tui` receives an image path, calls `hdim-core` to load the image and parse EXIF data.
2. **View:** `hdim-render` takes the core image data and generates a view suitable for the current terminal size.
3. **Interact:** User input acts on `hdim-tui` components (e.g., `crop.rs`).
4. **Modify:** Components send commands to `hdim-core` to mutate the state.
5. **Verify:** Changes are rendered back via the render pipeline.

## Roadmap

### Phase 1: Core Foundation & Metadata (Current)

- [x] Workspace setup.
- [x] **hdim-core**: EXIF data parsing implementation.
- [x] **hdim-render**: Basic pixel/view logic and snapshot testing.
- [x] **hdim-tui**: Basic App struct, Event Loop, and Image Loading.

### Phase 2: Rendering & Basic TUI

- [ ] **hdim-tui**: Integrate `hdim-render` logic into `ui.rs` to display the image.
- [ ] **hdim-tui**: Connect `exif_view.rs` to the render loop.
- [ ] **hdim-render**: Optimize rendering for high-res images (downscaling strategies).

### Phase 3: Manipulation Components

- [ ] **hdim-tui**: Complete `components/crop.rs` UI interaction.
- [ ] **hdim-core**: Implement actual crop logic backing the UI.
- [ ] **hdim-core**: Implement Rotate and Flip logic.
- [ ] **hdim-tui**: Add UI components for Rotation/Flipping.

### Phase 4: Advanced Features

- [x] **hdim-core**: Color adjustments (brightness, contrast) using `palette`.
- [x] **hdim-core**: Undo/Redo stack.
- [x] **hdim-tui**: Export/Save dialogs.

### Phase 5: Advanced Features & Image Adjustments

- [x] **hdim-core**: Undo/Redo stack.
- [x] **hdim-tui**: Export/Save dialogs.
- [x] **hdim-core**: Implement the logic for the following adjustment sliders (Range: -100 to +100):
- [x] **hdim-tui**: Implement the components with using the logic from `hdim-core`
- [x] **Saturation** (using `palette`)
- [x] **Vibrance**
- [x] **Exposure**
- [x] **Brightness**
- [x] **Contrast**
- [x] **Warmth**
- [x] **Hue**
- [x] **Fade**
- [x] **Film Grain**
- [x] **Noise**

## Tools (Translation & Reference)

The following suite of tools—inspired by standard mobile editors—must be implemented within the `hdim-core` manipulation logic and exposed via `hdim-tui` sliders:

| Tool           | Description                                              | Range        |
| -------------- | -------------------------------------------------------- | ------------ |
| **Saturation** | Intensity of colors.                                     | -100 to +100 |
| **Vibrance**   | Smart saturation (protects skin tones/saturated pixels). | -100 to +100 |
| **Exposure**   | Overall light captured in the image.                     | -100 to +100 |
| **Brightness** | Overall lightness of the image.                          | -100 to +100 |
| **Contrast**   | Difference between light and dark areas.                 | -100 to +100 |
| **Warmth**     | Shift between blue (cool) and yellow (warm) tones.       | -100 to +100 |
| **Hue**        | Shifts the entire color spectrum.                        | -100 to +100 |
| **Fade**       | Reduces contrast in shadows for a "matte" look.          | -100 to +100 |
| **Film Grain** | Adds simulated analog texture.                           | -100 to +100 |
| **Noise**      | Adds digital luminance/chroma noise.                     | -100 to +100 |

## Tech Stack

- **Language:** Rust (2021 Edition)
- **TUI:** `ratatui`, `crossterm`
- **Image Processing:** `image` (0.25), `palette`, `ansi-to-tui`
- **Metadata:** `kamadak-exif`
- **Error Handling:** `anyhow` (app), `thiserror` (lib), `color-eyre` (panic handling)
- **Testing:** `insta` (snapshots)

## Coding Standards

- **Formatting:** Strictly run `cargo fmt` on every change.
- **Snapshots:** When changing rendering logic, run `cargo insta review` to accept/reject snapshot changes.
- **Errors:** Use `color-eyre` for pretty panics in the binary, `thiserror` for internal library errors.
- **Tool Retention:** Existing tools and features must not be removed when adding new adjustments. All legacy manipulation logic must remain functional or be refactored to support the new slider system.
- **Functional Atomicity:** Functions should be designed to perform a single, atomic task. Avoid "God functions." If a public function or UI render method becomes complex, decompose the logic into private helper functions. This ensures the code remains testable and readable without enforcing arbitrary line-count limits.
- **Args:** Minimal parsing; expecting only `[IMAGE_PATH]` via `std::env::args`.
