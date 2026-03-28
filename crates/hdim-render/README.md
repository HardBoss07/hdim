# hdim-render

Terminal rendering logic for HDIM (High Definition Image Manipulator) using half-block characters.

This crate handles the translation of [DynamicImage] buffers into ANSI escape sequences suitable for display in a terminal emulator. It uses "half-block" characters (`▄`) to effectively double the vertical resolution of the terminal grid.

Part of the [hdim](https://github.com/HardBoss07/hdim project.
