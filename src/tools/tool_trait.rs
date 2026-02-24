//! Tool trait - interface for all tools

use crate::app::AppState;
use crate::data::Geometry;
use egui::Ui;
use winit::event::MouseButton;

/// Context passed to tools
pub struct ToolContext<'a> {
    pub app_state: &'a mut AppState,
    pub mouse_position: (f32, f32),
    pub mouse_delta: (f32, f32),
    pub mouse_button: Option<MouseButton>,
}

/// Tool trait - all interactive tools must implement this
pub trait Tool {
    /// Get the tool name
    fn name(&self) -> &str;

    /// Render UI for the tool in the inspector panel
    fn ui(&mut self, ui: &mut Ui, _app_state: &mut AppState) {
        ui.label(self.name());
    }

    /// Handle mouse events
    fn handle_mouse(
        &mut self,
        _button: winit::event::MouseButton,
        _pressed: bool,
        _ctx: &mut ToolContext,
    ) {
    }

    /// Handle mouse move
    fn handle_mouse_move(
        &mut self,
        _position: (f32, f32),
        _delta: (f32, f32),
        _ctx: &mut ToolContext,
    ) {
    }

    /// Handle key events
    fn handle_key(
        &mut self,
        _key: winit::keyboard::Key,
        _pressed: bool,
        _ctx: &mut ToolContext,
    ) {
    }

    /// Update the tool (called every frame)
    fn update(&mut self, _dt: f32, _ctx: &mut ToolContext) {}

    /// Get preview geometry (for tools that have preview)
    fn preview(&self) -> Option<Geometry> {
        None
    }

    /// Commit the tool action and return a command
    fn commit(&mut self) -> Option<Box<dyn crate::app::Command>> {
        None
    }

    /// Check if the tool has a pending commit
    fn has_pending_commit(&self) -> bool {
        false
    }
}