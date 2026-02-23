//! Toolbar - tool selection UI

use crate::app::AppState;
use crate::app::ActiveTool;
use egui::Ui;

/// Toolbar for tool selection
pub struct Toolbar {
    /// Whether the toolbar is visible
    pub visible: bool,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            visible: true,
        }
    }

    /// Render the toolbar
    pub fn render(&mut self, ui: &mut Ui, app_state: &mut AppState) {
        if !self.visible {
            return;
        }

        // Top toolbar
        egui::TopBottomPanel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tools:");
                
                // Select tool button
                let select_selected = app_state.active_tool == ActiveTool::Select;
                if ui.selectable_label(select_selected, "Select").clicked() {
                    app_state.set_tool(ActiveTool::Select);
                }
                
                // FitPlane tool button
                let fitplane_selected = app_state.active_tool == ActiveTool::FitPlane;
                if ui.selectable_label(fitplane_selected, "Fit Plane").clicked() {
                    app_state.set_tool(ActiveTool::FitPlane);
                }
                
                ui.separator();
                
                // Camera mode indicator
                let mode_text = match app_state.camera.mode {
                    crate::viewer::camera::CameraMode::Orbit => "Orbit",
                    crate::viewer::camera::CameraMode::FirstPerson => "FPS",
                };
                ui.label(format!("Camera: {}", mode_text));
                
                // Toggle camera mode
                if ui.button("Tab").clicked() {
                    app_state.toggle_camera_mode();
                }
                
                ui.separator();
                
                // Undo/Redo
                let can_undo = app_state.history.can_undo();
                let can_redo = app_state.history.can_redo();
                
                if ui.add_enabled(can_undo, egui::Button::new("Undo")).clicked() {
                    app_state.history.undo(&mut app_state.scene);
                }
                
                if ui.add_enabled(can_redo, egui::Button::new("Redo")).clicked() {
                    app_state.history.redo(&mut app_state.scene);
                }
            });
        });
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}