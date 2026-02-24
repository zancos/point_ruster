//! Application state - holds all mutable state for the editor

use crate::data::Scene;
use crate::viewer::camera::Camera;
use crate::tools::{Tool, tool_select::ToolSelect, tool_fit_plane::ToolFitPlane};
use crate::app::history::History;

/// Active tool enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTool {
    Select,
    FitPlane,
}

/// Main application state
pub struct AppState {
    pub scene: Scene,
    pub camera: Camera,
    pub active_tool: ActiveTool,
    pub tool_select: ToolSelect,
    pub tool_fit_plane: ToolFitPlane,
    pub history: History,
    pub point_size: f32,
    pub show_ui: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            camera: Camera::new(),
            active_tool: ActiveTool::Select,
            tool_select: ToolSelect::new(),
            tool_fit_plane: ToolFitPlane::new(),
            history: History::new(),
            point_size: 2.0,
            show_ui: true,
        }
    }

    /// Get the currently active tool
    pub fn current_tool(&mut self) -> &mut dyn Tool {
        match self.active_tool {
            ActiveTool::Select => &mut self.tool_select,
            ActiveTool::FitPlane => &mut self.tool_fit_plane,
        }
    }

    /// Toggle between camera modes
    pub fn toggle_camera_mode(&mut self) {
        self.camera.toggle_mode();
    }

    /// Set active tool
    pub fn set_tool(&mut self, tool: ActiveTool) {
        self.active_tool = tool;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}