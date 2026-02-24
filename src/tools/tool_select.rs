//! Select tool - point selection with click and box select

use crate::app::commands::CmdSetSelection;
use crate::app::AppState;
use crate::tools::tool_trait::{Tool, ToolContext};
use crate::viewer::picking::Picking;
use egui::Ui;
use winit::event::MouseButton;

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectMode {
    /// Single point selection
    Point,
    /// Box selection
    Box,
}

/// Select tool for picking points
pub struct ToolSelect {
    /// Current selection mode
    pub mode: SelectMode,
    /// Box selection start (screen coordinates)
    box_start: Option<(f32, f32)>,
    /// Box selection end (screen coordinates)
    box_end: Option<(f32, f32)>,
    /// Is box selection active
    is_box_selecting: bool,
    /// Picking helper
    picking: Picking,
}

impl ToolSelect {
    pub fn new() -> Self {
        Self {
            mode: SelectMode::Point,
            box_start: None,
            box_end: None,
            is_box_selecting: false,
            picking: Picking::new(),
        }
    }

    /// Get the current box selection rectangle
    pub fn box_rect(&self) -> Option<((f32, f32), (f32, f32))> {
        match (self.box_start, self.box_end) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
        }
    }

    /// Clear box selection
    pub fn clear_box(&mut self) {
        self.box_start = None;
        self.box_end = None;
        self.is_box_selecting = false;
    }
}

impl Tool for ToolSelect {
    fn name(&self) -> &str {
        "Select"
    }

    fn ui(&mut self, ui: &mut Ui, _app_state: &mut AppState) {
        ui.label("Selection Mode:");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.mode, SelectMode::Point, "Point");
            ui.selectable_value(&mut self.mode, SelectMode::Box, "Box");
        });
        
        ui.separator();
        ui.label("Controls:");
        ui.label("- Click: Select point");
        ui.label("- Drag: Box select");
        ui.label("- Shift+Click: Add to selection");
    }

    fn handle_mouse(
        &mut self,
        button: MouseButton,
        pressed: bool,
        ctx: &mut ToolContext,
    ) {
        if button != MouseButton::Left {
            return;
        }

        let app_state = ctx.app_state;

        if pressed {
            // Mouse pressed - start selection
            match self.mode {
                SelectMode::Point => {
                    // Single point selection
                    if let Some(result) = self.picking.pick_point(
                        &app_state.scene.cloud,
                        &app_state.camera,
                        ctx.mouse_position.0,
                        ctx.mouse_position.1,
                    ) {
                        let mut selection = app_state.scene.selection.selected_points();
                        
                        if ctx.app_state.scene.selection.is_point_selected(result.point_index) {
                            // Already selected, maybe deselect
                            if !ctx.mouse_button.map(|b| b == MouseButton::Left).unwrap_or(false) {
                                // Shift not held, replace selection
                                selection.clear();
                                selection.push(result.point_index);
                            }
                        } else {
                            selection.push(result.point_index);
                        }
                        
                        let old_selection = app_state.scene.selection.selected_points();
                        let cmd = CmdSetSelection::new(selection, old_selection);
                        app_state.history.execute(Box::new(cmd), &mut app_state.scene);
                    }
                }
                SelectMode::Box => {
                    // Start box selection
                    self.box_start = Some(ctx.mouse_position);
                    self.box_end = Some(ctx.mouse_position);
                    self.is_box_selecting = true;
                }
            }
        } else {
            // Mouse released - finish box selection
            if self.is_box_selecting {
                if let (Some(start), Some(end)) = (self.box_start, self.box_end) {
                    let min_x = start.0.min(end.0);
                    let max_x = start.0.max(end.0);
                    let min_y = start.1.min(end.1);
                    let max_y = start.1.max(end.1);

                    // Only select if box has some size
                    if (max_x - min_x).abs() > 0.01 && (max_y - min_y).abs() > 0.01 {
                        let selected = self.picking.box_select(
                            &app_state.scene.cloud,
                            &app_state.camera,
                            min_x,
                            min_y,
                            max_x,
                            max_y,
                        );

                        let old_selection = app_state.scene.selection.selected_points();
                        let mut new_selection = old_selection.clone();
                        new_selection.extend(selected);
                        
                        let cmd = CmdSetSelection::new(new_selection, old_selection);
                        app_state.history.execute(Box::new(cmd), &mut app_state.scene);
                    }
                }
                
                self.clear_box();
            }
        }
    }

    fn handle_mouse_move(
        &mut self,
        position: (f32, f32),
        _delta: (f32, f32),
        _ctx: &mut ToolContext,
    ) {
        if self.is_box_selecting {
            self.box_end = Some(position);
        }
    }
}

impl Default for ToolSelect {
    fn default() -> Self {
        Self::new()
    }
}

// Helper to get camera reference from app state
fn get_camera(app_state: &crate::app::AppState) -> &crate::viewer::Camera {
    &app_state.camera
}