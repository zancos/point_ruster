//! PLY Editor - Point cloud viewer/editor with tool-based geometry creation
//! 
//! A Rust application for viewing and editing PLY point clouds with:
//! - Dual camera modes (Orbit CAD + FPS)
//! - Selection tools (click + box select)
//! - FitPlane tool for geometry creation
//! - Undo/redo support
//! - OBJ export

mod app;
mod data;
mod viewer;
mod tools;
mod ui;
mod io;
mod math;

use app::AppState;
use std::path::PathBuf;
use log::info;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    info!("Starting PLY Editor");
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let ply_path = if args.len() > 1 {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };
    
    // Create and run the application
    let mut app_state = AppState::new();
    
    if let Some(path) = ply_path {
        info!("Loading PLY file: {:?}", path);
        match io::ply_loader::load_ply(&path) {
            Ok(cloud) => {
                app_state.scene.load_point_cloud(cloud);
                info!("PLY loaded successfully");
            }
            Err(e) => {
                log::error!("Failed to load PLY: {}", e);
            }
        }
    }
    
    // Run the application (this will block)
    app::run(app_state);
}