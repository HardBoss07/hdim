# Contributing to hdim

Thank you for your interest in contributing to `hdim`! To maintain code quality and project consistency, please adhere to the following guidelines.

## General Principles

- **Code Style:** We follow standard Rust idioms.
- **Atomic Changes:** Keep pull requests focused on a single feature or bug fix.
- **Documentation:** All public functions, structs, and enums must have Rustdoc comments.

## Mandatory Workflow

1.  **Formatting:** Always run `cargo fmt` before committing your changes.
2.  **Structural Changes:** If you add, move, or rename files/directories, you **must** update the `Project Structure.md` file to reflect the new state.
3.  **Testing:**
    - For `hdim-core`, add unit tests where appropriate.
    - For `hdim-render`, we use snapshot testing via `insta`. If your changes affect rendering output, run `cargo insta review` to verify and accept the new snapshots.
4.  **Compiler Checks:** Ensure your code is free of warnings and errors by running `cargo check` and `cargo clippy`.

## Coding Standards

- **Error Handling:**
  - Use `thiserror` for internal library errors in `hdim-core`.
  - Use `color-eyre` for application-level error handling and pretty panics in `hdim-tui`.
- **Naming Conventions:**
  - Do not use abbreviations for variable names (e.g., use `image` instead of `img`, `index` instead of `idx`, `width` instead of `w`).
  - Standard short forms for coordinates (`x`, `y`) and colors (`r`, `g`, `b`) are acceptable.
- **Visuals:** Avoid using emojis in code comments or UI text.
- **Functional Atomicity:** Design functions to perform a single, atomic task. Decompose complex logic into private helper functions to maintain readability and testability.
- **Tool Retention:** When refactoring or adding new features, ensure all existing image manipulation tools remain functional.

## Tech Stack Reference

- **TUI:** `ratatui`, `crossterm`
- **Image Processing:** `image` (0.25), `palette`, `ansi-to-tui`
- **Metadata:** `kamadak-exif`
- **Testing:** `insta` (snapshots)

By following these guidelines, you help us keep `hdim` a robust and maintainable tool for terminal-based image manipulation.
