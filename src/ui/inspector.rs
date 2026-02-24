//! Inspector - tool parameters and info panel

use crate::app::AppState;
use egui::Ui;

/// Inspector panel for tool parameters
pub struct Inspector {
    /// Whether the inspector is visible
    pub visible: bool,
}

impl Inspector {
    pub fn new() -> Self {
        Self {
            visible: true,
        }
    }

    /// Render the inspector panel
    pub fn render(&mut self, ui: &mut Ui, app_state: &mut AppState) {
        if !self.visible {
            return;
        }

        egui::SidePanel::right("inspector").default_width(250.0).show_inside(ui, |ui| {
            ui.heading("Inspector");
            ui.separator();
            
            // Show current tool UI
            let tool = app_state.current_tool();
            ui.label(egui::RichText::new(tool.name()).heading());
            ui.separator();
            
            // Tool-specific UI
            tool.ui(ui, &mut *app_state);
            
            ui.separator();
            
            // Selection info
            ui.label("Selection:");
            let point_count = app_state.scene.selection.point_count();
            let geo_count = app_state.scene.selection.geometry_count();
            ui.label(format!("  Points: {}", point_count));
            ui.label(format!("  Geometries: {}", geo_count));
            
            ui.separator();
            
            // Point cloud info
            if app_state.scene.has_cloud() {
                ui.label("Point Cloud:");
                let cloud = &app_state.scene.cloud;
                ui.label(format!("  Points: {}", cloud.len()));
                let size = cloud.size();
                ui.label(format!("  Size: {:.2} x {:.2} x {:.2}", size.x, size.y, size.z));
            }
        });
    }
}

impl Default for Inspector {
    fn default() -> Self {
        Self::new()
    }
}