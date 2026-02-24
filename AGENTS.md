# AGENTS.md

Project: point_ruster (Rust)

## GOAL

Build a PLY point cloud viewer/editor with tool-based geometry creation (plane fit to surface), dual navigation modes (Orbit CAD + FPS), selection, preview, and undo/redo.

## RULES

- Keep a clean architecture: app/ state, data/ scene, viewer/ rendering, tools/ algorithms+interaction, ui/ panels.
- All scene mutations go through Command objects for undo/redo.
- Tools must not directly render; they return preview geometry and/or commands.
- Prefer deterministic algorithms; document numeric tolerances.
- Add small tests for math-heavy parts.
- Keep compilation on stable Rust. Run cargo fmt + cargo clippy.
- Avoid huge monolithic files; keep modules small and cohesive.

## DEFINITION OF DONE (MVP)

- Load and render PLY as points.
- Orbit + FPS camera.
- Click select + box select, highlight selection.
- FitPlane tool with preview and commit.
- Undo/redo for selection changes and geometry creation.
- Export created meshes to OBJ.
- README includes build/run instructions and keybindings.