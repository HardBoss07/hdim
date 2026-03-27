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
  - `transform.rs`: Transform tool logic (formerly crop.rs).
  - `exif_view.rs`: EXIF data display widget.

  - `src/app.rs`: Application state (`AppMode`, `ActiveWidget`), zoom/scroll logic, and lifecycle management.
  - `src/events.rs`: Input event handling loop.
  - `src/ui.rs`: Main drawing logic using `ratatui`.
  - `src/main.rs`: Entry point, manual argument parsing (image path).

## Data Flow

1. **Load:** `hdim-tui` receives an image path, calls `hdim-core` to load the image and parse EXIF data.
2. **View:** `hdim-render` takes the core image data and generates a view suitable for the current terminal size.
3. **Interact:** User input acts on `hdim-tui` components.
4. **Modify:** Components send commands to `hdim-core` to mutate the state.
5. **Verify:** Changes are rendered back via the render pipeline.

## Roadmap

### Phase 1: Core Foundation & Metadata

- [x] Workspace setup.
- [x] **hdim-core**: EXIF data parsing implementation.
- [x] **hdim-render**: Basic pixel/view logic and snapshot testing.
- [x] **hdim-tui**: Basic App struct, Event Loop, and Image Loading.

### Phase 2: Rendering & Basic TUI

- [x] **hdim-tui**: Integrate `hdim-render` logic into `ui.rs` to display the image.
- [x] **hdim-tui**: Connect `exif_view.rs` to the render loop.
- [x] **hdim-render**: Optimize rendering for high-res images (downscaling strategies).

### Phase 3: Transform Tool Overhaul (Replacing Crop Tool)

- [x] **hdim-tui**: Rename the Crop tool to "Transform Tool" (`components/transform.rs`).
- [x] **hdim-core**: Implement stackable 90° rotation logic (Left/Right) resulting in 90/180/270°.
- [x] **hdim-core**: Implement horizontal and vertical flipping logic.
- [x] **hdim-core**: Implement relative (%) and absolute (px) cropping (detect % in input).
- [x] **hdim-core & hdim-render**: Implement "Crop from viewport" (calculate absolute crop bounds based on current camera view).
- [x] **hdim-tui**: Render a consistent visual indicator (line) for where the viewport crop will cut off.
- [x] **hdim-core**: Implement the final "Apply Crop" execution.
- [x] **hdim-tui**: Retain all legacy crop features within the new Transform Tool UI.

### Phase 4: Color Adjustments & Base Undo/Redo

- [x] **hdim-core**: Color adjustments (brightness, contrast) using `palette`.
- [x] **hdim-core**: Base Undo/Redo stack setup.
- [x] **hdim-tui**: Export/Save dialogs.

### Phase 5: Advanced Features & Image Adjustments

- [x] **hdim-core**: Implement the logic for the following adjustment sliders (Range: -100 to +100).
- [x] **hdim-tui**: Implement the components using the logic from `hdim-core`.
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

### Phase 6: UI/UX Refinements & Safety

- [x] **hdim-tui**: Change quit keybind from `q` to `Ctrl+q`.
- [x] **hdim-tui**: Implement a warning popup before exiting if there are unsaved changes.
- [x] **hdim-tui**: Enforce a hard zoom limit of 2x to prevent rendering breakdowns.
- [x] **hdim-tui**: Clean up `theme.rs` by implementing the currently unused fields or removing them entirely.

### Phase 7: Configuration & Localization

- [x] **hdim-core**: Architect a strongly defined localization sub-module. Create a uniform base type for all application text.
- [x] **hdim-core**: Implement English (Default) and German language structs using the uniform type.
- [x] **hdim-tui**: Create a persistent configuration file system.
- [x] **hdim-tui**: Build a Settings UI to allow users to switch themes and select their preferred language.

### Phase 8: Export Polish & Adjustment Tweaks

- [x] **hdim-core**: Overhaul EXIF data handling on export to optionally strip sensitive/unnecessary data while preserving dimensions.
- [x] **hdim-core**: Refine **Noise** slider logic to exclusively add random RGB noise.
- [x] **hdim-core**: Refine **Film Grain** slider logic to exclusively add random gray/luminance noise.
- [x] **Documentation**: Adjust the modifier values and tool descriptions in the `GEMINI.md` README.

### Phase 9: Global State & Keybinds

- [x] **hdim-core & hdim-tui**: Overhaul the Undo/Redo stack to be a global application feature, rather than limited to the adjustments tab.
- [x] **hdim-tui**: Allow `Ctrl+s` to open the save prompt globally from anywhere in the app (handle potential conflicts with undo/redo states).

### Phase 10: Bug Fixes & UI Consistency

- [x] **hdim-tui**: Fix keybind overlap so typing values (like '1') in adjustment settings does not trigger tool shortcuts.
- [x] **hdim-tui**: Enable holding `Ctrl` while adjusting tool values to increment/decrement by 10 directly.
- [x] **hdim-tui**: Add "No Metadata found" placeholder text in EXIF view when metadata is empty.
- [x] **hdim-tui**: Remove FULL CAPS LOCK on all UI labels (excluding app state / app name labels).
- [x] **hdim-tui**: Fix top label cropping in the Adjustment sidebar caused by overlap with the main title.
- [x] **hdim-tui**: Standardize border types (rounded vs. sharp) across all sidebar tools to ensure visual consistency.
- [x] **hdim-tui**: Implement "Soft Apply" for Transform options: dynamically update the viewport during adjustment, but require an explicit "Apply" action to finalize (including a warning popup for unapplied changes).

### Phase 11: Performance & Memory Optimization

- [ ] **hdim-tui**: Fix "sticky" inputs (dropping keystrokes) while explicitly keeping the existing time scheduler intact to preserve exact number typing.
- [ ] **hdim-render**: Optimize rendering to only occur when a change happens, eliminating the slowdown caused by recalculating every adjustment every frame.
- [ ] **hdim-tui & hdim-render**: Investigate and fix the continuous 1-2MB/s memory leak while idling on an image (evaluate if this is a ratatui/crossterm issue with PowerShell or an internal allocation loop).

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
| **Fade**       | Reduces contrast in shadows for a "matte" look.          | 0 to +100    |
| **Film Grain** | Adds achromatic (gray) luminance noise.                  | 0 to +100    |
| **Noise**      | Adds digital RGB noise.                                  | 0 to +100    |

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
