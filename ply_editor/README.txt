PLY Editor (Rust)
=================

Build:
cargo build

Run:
cargo run -- path/to/cloud.ply

Controls:
--------
Tab: toggle camera mode (Orbit/FPS)

Orbit mode:
- LMB drag: rotate
- MMB drag: pan
- Wheel: zoom

FPS mode:
- Mouse move: look
- W/A/S/D: move
- Q/E: down/up
- Wheel: forward/back (optional)

Tools:
------
Select:
- Click: select nearest point
- Drag: box select

FitPlane:
- Requires selected points
- Adjust tolerance in inspector
- Apply/Commit button creates geometry

Undo/Redo:
----------
Ctrl+Z / Ctrl+Y

Export:
-------
File -> Export OBJ (or keybinding)