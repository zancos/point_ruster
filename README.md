# Point Ruster

A PLY point cloud viewer/editor with tool-based geometry creation, dual navigation modes (Orbit CAD + FPS), selection, preview, and undo/redo.

## Features

- Load and render PLY point clouds
- Dual camera modes: Orbit CAD and FPS
- Selection tools: click select and box select
- FitPlane tool for geometry creation from selected points
- Undo/redo support for all scene mutations
- Export created meshes to OBJ format

## Build

```bash
cargo build
```

## Run

```bash
cargo run -- path/to/cloud.ply
```

## Controls

### Camera Modes
- **Tab**: Toggle between Orbit and FPS camera modes

### Orbit Mode
- **Left Mouse Button drag**: Rotate view
- **Middle Mouse Button drag**: Pan view
- **Mouse Wheel**: Zoom in/out

### FPS Mode
- **Mouse move**: Look around
- **W/A/S/D**: Move forward/left/backward/right
- **Q/E**: Move down/up
- **Mouse Wheel**: Move forward (optional)

### Tools

#### Select Tool
- **Click**: Select nearest point
- **Drag**: Box select multiple points

#### FitPlane Tool
- Select points first using the Select tool
- Adjust tolerance in the inspector panel
- Click "Apply" to preview the plane
- Click "Commit" to create the geometry

### Undo/Redo
- **Ctrl+Z**: Undo
- **Ctrl+Y**: Redo

### Export
- **File → Export OBJ**: Export created meshes to OBJ format

## Architecture

- `app/` - Application state, commands, and history
- `data/` - Scene data structures (point clouds, geometry, selection)
- `viewer/` - Rendering and camera systems
- `tools/` - Interactive tools (selection, plane fitting)
- `ui/` - Egui-based UI panels
- `io/` - PLY and OBJ file I/O
- `math/` - Mathematical utilities (PCA, basis computation)

## License

MIT