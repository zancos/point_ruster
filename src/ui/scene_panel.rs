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
                // Collect geometry info first to avoid borrow issues
                let geometry_info: Vec<_> = app_state.scene.geometries.iter()
                    .map(|obj| (obj.id, obj.visible, obj.name.clone(), format!("{}", obj.geometry)))
                    .collect();
                
                for (id, visible, name, type_name) in geometry_info {
                    ui.horizontal(|ui| {
                        // Visibility toggle
                        let vis_text = if visible { "👁" } else { "○" };
                        if ui.button(vis_text).clicked() {
                            let cmd = CmdToggleVisibility::new(id, visible);
                            app_state.history.execute(Box::new(cmd), &mut app_state.scene);
                        }
                        
                        // Geometry name and type
                        ui.label(format!("{} ({})", name, type_name));
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