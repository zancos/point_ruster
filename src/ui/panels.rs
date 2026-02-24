//! UI panels - main UI structure

use crate::app::AppState;
use egui::Context;
use crate::ui::toolbar::Toolbar;
use crate::ui::inspector::Inspector;
use crate::ui::scene_panel::ScenePanel;

/// Main UI structure
pub struct UI {
    pub toolbar: Toolbar,
    pub inspector: Inspector,
    pub scene_panel: ScenePanel,
}

impl UI {
    pub fn new() -> Self {
        Self {
            toolbar: Toolbar::new(),
            inspector: Inspector::new(),
            scene_panel: ScenePanel::new(),
        }
    }

    /// Render the UI
    pub fn render(&mut self, app_state: &mut AppState, _window: &winit::window::Window) {
        // For now, we'll use a simple immediate mode approach
        // In a full implementation, we'd integrate with winit and wgpu properly
        
        // This is a placeholder - the actual rendering would be done
        // through egui-wgpu with the render loop
    }

    /// Create egui context for rendering
    pub fn create_context(&self) -> Context {
        Context::default()
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}