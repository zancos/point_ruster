//! FitPlane tool - fit a plane to selected points

use crate::app::commands::{CmdAddGeometry, Command};
use crate::app::AppState;
use crate::data::geometry::{Geometry, PlanePrimitive};
use crate::tools::tool_trait::{Tool, ToolContext};
use crate::tools::plane_fit;
use egui::Ui;

/// Fit plane tool
pub struct ToolFitPlane {
    /// Tolerance for plane fitting
    pub tolerance: f32,
    /// Minimum number of points required
    pub min_points: usize,
    /// Whether there's a pending commit
    has_commit: bool,
    /// Preview plane (if any)
    preview_plane: Option<PlanePrimitive>,
}

impl ToolFitPlane {
    pub fn new() -> Self {
        Self {
            tolerance: 0.1,
            min_points: 3,
            has_commit: false,
            preview_plane: None,
        }
    }

    /// Update the preview based on current selection
    pub fn update_preview(&mut self, app_state: &mut AppState) {
        let selected = app_state.scene.selection.selected_points();
        
        if selected.len() < self.min_points {
            self.preview_plane = None;
            return;
        }

        // Get selected points
        let points: Vec<glam::Vec3> = selected
            .iter()
            .filter_map(|&i| app_state.scene.cloud.get(i))
            .collect();

        if points.len() < self.min_points {
            self.preview_plane = None;
            return;
        }

        // Fit plane
        if let Some(plane_result) = plane_fit::fit_plane(&points, self.tolerance) {
            // Create a plane primitive for preview
            let center = plane_result.centroid;
            let normal = plane_result.normal;
            
            // Compute bounding box of inliers
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;
            
            for &idx in &plane_result.inliers {
                if let Some(p) = points.get(idx) {
                    let local = *p - center;
                    let u = if normal.abs().y < 0.99 { glam::Vec3::Y } else { glam::Vec3::X };
                    let right = glam::Vec3::cross(normal, u).normalize();
                    let up = glam::Vec3::cross(right, normal).normalize();
                    
                    let x = local.dot(right);
                    let y = local.dot(up);
                    
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                }
            }
            
            let width = max_x - min_x;
            let height = max_y - min_y;
            
            self.preview_plane = Some(PlanePrimitive::new(center, normal, width, height));
        } else {
            self.preview_plane = None;
        }
    }
}

impl Tool for ToolFitPlane {
    fn name(&self) -> &str {
        "Fit Plane"
    }

    fn ui(&mut self, ui: &mut Ui, app_state: &mut AppState) {
        ui.label("Fit Plane Tool");
        ui.separator();
        
        ui.add(egui::Slider::new(&mut self.tolerance, 0.01..=1.0).text("Tolerance"));
        ui.add(egui::Slider::new(&mut self.min_points, 3..=100).text("Min Points"));
        
        let selected_count = app_state.scene.selection.point_count();
        ui.separator();
        ui.label(format!("Selected points: {}", selected_count));
        
        if selected_count < self.min_points {
            ui.label(egui::RichText::new("Select more points").color(egui::Color32::RED));
        } else {
            if ui.button("Apply").clicked() {
                self.has_commit = true;
                self.update_preview(app_state);
            }
        }
        
        ui.separator();
        ui.label("Controls:");
        ui.label("- Adjust tolerance in slider");
        ui.label("- Click Apply to create plane");
    }

    fn preview(&self) -> Option<Geometry> {
        self.preview_plane.clone().map(Geometry::PlanePrimitive)
    }

    fn commit(&mut self) -> Option<Box<dyn Command>> {
        if self.has_commit {
            self.has_commit = false;
            
            if let Some(plane) = &self.preview_plane {
                let geometry = Geometry::PlanePrimitive(plane.clone());
                return Some(Box::new(CmdAddGeometry::new(geometry)));
            }
        }
        None
    }

    fn has_pending_commit(&self) -> bool {
        self.has_commit
    }
}

impl Default for ToolFitPlane {
    fn default() -> Self {
        Self::new()
    }
}