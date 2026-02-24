//! Application run loop - initializes and runs the main event loop

use crate::app::AppState;
use crate::viewer::Renderer;
use crate::ui::UI;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};
use log::{info, error};

/// Run the application with the given state
pub fn run(mut app_state: AppState) {
    // Create the event loop
    let event_loop = EventLoop::new();
    
    // Create the window
    let window = WindowBuilder::new()
        .with_title("Point Ruster")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(event_loop.as_ref().unwrap())
        .expect("Failed to create window");
    
    // Initialize renderer
    let mut renderer = match Renderer::new(&window) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to initialize renderer: {}", e);
            return;
        }
    };
    
    // Initialize UI
    let mut ui = UI::new();
    
    info!("Application initialized successfully");
    
    // Run the event loop
    event_loop.expect("Failed to create event loop").run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::CloseRequested, 
                .. 
            } => {
                window_target.exit();
            }
            winit::event::Event::WindowEvent { 
                event: winit::event::WindowEvent::Resized(size), 
                .. 
            } => {
                renderer.resize(size.width, size.height);
            }
            winit::event::Event::AboutToWait => {
                // Update
                // (Add update logic here)
                
                // Render
                let output = renderer.render(&app_state);
                ui.render(&mut app_state, &window);
                
                // Present
                if let Some(output) = output {
                    renderer.present(output);
                }
            }
            _ => {}
        }
    });
}