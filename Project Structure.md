# Project Structure

```
hdim/
├── crates/
│   ├── hdim-core/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   ├── tests/
│   │   │   └── resizing.rs
│   │   └── Cargo.toml
│   ├── hdim-render/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── pixel.rs
│   │   │   └── view.rs
│   │   ├── tests/
│   │   │   ├── images/
│   │   │   │   ├── 4k.jpg
│   │   │   │   └── WindowsXP.png
│   │   │   ├── snapshots/
│   │   │   │   ├── complex_image__complex_render.snap
│   │   │   │   ├── image_file__render_real_image_snapshot.snap
│   │   │   │   ├── image_file__render_real_image_snapshot_size_2.snap
│   │   │   │   ├── image_file__render_real_image_snapshot_size_4.snap
│   │   │   │   ├── image_file__render_real_image_snapshot_size_8.snap
│   │   │   │   └── visuals__render_snapshot.snap
│   │   │   ├── complex_image.rs
│   │   │   ├── image_file.rs
│   │   │   └── visuals.rs
│   │   └── Cargo.toml
│   └── hdim-tui/
│       ├── src/
│       │   ├── app.rs
│       │   ├── events.rs
│       │   ├── main.rs
│       │   └── ui.rs
│       └── Cargo.toml
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── Project Structure.md
└── README.md
```