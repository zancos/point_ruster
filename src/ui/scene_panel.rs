//! Scene panel - list of geometries in the scene

use crate::app::AppState;
use crate::app::commands::CmdToggleVisibility;
use egui::Ui;

/// Scene panel showing geometries
pub struct ScenePanel {
    /// Whether the panel is visible
    pub visible: bool,
}

impl ScenePanel {
    pub fn new() -> Self {
        Self {
            visible: true,
        }
    }

    /// Render the scene panel
    pub fn render(&mut self, ui: &mut Ui, app_state: &mut AppState) {
        if !self.visible {
            return;
        }

        egui::SidePanel::left("scene_panel").default_width(200.0).show_inside(ui, |ui| {
            ui.heading("Scene");
            ui.separator();
            
            // Point cloud
            if app_state.scene.has_cloud() {
                ui.horizontal(|ui| {
                    let cloud = &app_state.scene.cloud;
                    ui.label(format!("Cloud ({} pts)", cloud.len()));
                });
            } else {
                ui.label("No cloud loaded");
            }
            
            ui.separator();
            
            // Geometries
            ui.label("Geometries:");
            
            if app_state.scene.geometries.is_empty() {
                ui.label("  (none)");
            } else {
                for obj in &app_state.scene.geometries {
                    ui.horizontal(|ui| {
                        // Visibility toggle
                        let vis_text = if obj.visible { "👁" } else { "○" };
                        if ui.button(vis_text).clicked() {
                            let cmd = CmdToggleVisibility::new(obj.id, obj.visible);
                            app_state.history.execute(Box::new(cmd), &mut app_state.scene);
                        }
                        
                        // Geometry name and type
                        let type_name = format!("{}", obj.geometry);
                        ui.label(format!("{} ({})", obj.name, type_name));
                    });
                }
            }
            
            ui.separator();
            
            // Export button
            if ui.button("Export OBJ...").clicked() {
                // TODO: Open file dialog for export
            }
        });
    }
}

impl Default for ScenePanel {
    fn default() -> Self {
        Self::new()
    }
}