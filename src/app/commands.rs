//! Commands - Command pattern for undo/redo support

use crate::data::{Scene, Geometry, SceneObject};
use std::any::Any;
use std::fmt;

/// Command trait for undo/redo support
pub trait Command: fmt::Debug {
    /// Apply the command to the scene
    fn apply(&mut self, scene: &mut Scene);
    
    /// Undo the command
    fn undo(&mut self, scene: &mut Scene);
    
    /// Get a description of the command
    fn description(&self) -> &str;
    
    /// Get as Any for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Get as mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: fmt::Debug + 'static> Command for T
where
    T: FnMut(&mut Scene),
{
    fn apply(&mut self, scene: &mut Scene) {
        self(scene);
    }
    
    fn undo(&mut self, _scene: &mut Scene) {
        // Default implementation does nothing
        // Override in specific commands
    }
    
    fn description(&self) -> &str {
        "Command"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Command to set the selection
#[derive(Debug, Clone)]
pub struct CmdSetSelection {
    pub new_selection: Vec<usize>,
    pub old_selection: Vec<usize>,
    description: String,
}

impl CmdSetSelection {
    pub fn new(new_selection: Vec<usize>, old_selection: Vec<usize>) -> Self {
        Self {
            new_selection,
            old_selection,
            description: "Set Selection".to_string(),
        }
    }
}

impl Command for CmdSetSelection {
    fn apply(&mut self, scene: &mut Scene) {
        scene.selection.set_points(self.new_selection.clone());
    }
    
    fn undo(&mut self, scene: &mut Scene) {
        scene.selection.set_points(self.old_selection.clone());
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Command to add geometry to the scene
#[derive(Debug, Clone)]
pub struct CmdAddGeometry {
    pub geometry: Geometry,
    pub id: Option<u64>,
    description: String,
}

impl CmdAddGeometry {
    pub fn new(geometry: Geometry) -> Self {
        Self {
            geometry,
            id: None,
            description: "Add Geometry".to_string(),
        }
    }
}

impl Command for CmdAddGeometry {
    fn apply(&mut self, scene: &mut Scene) {
        let obj = SceneObject::new(self.geometry.clone());
        self.id = Some(obj.id);
        scene.add_geometry(obj);
    }
    
    fn undo(&mut self, scene: &mut Scene) {
        if let Some(id) = self.id {
            scene.remove_geometry(id);
        }
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Command to toggle visibility of a geometry
#[derive(Debug, Clone)]
pub struct CmdToggleVisibility {
    pub geometry_id: u64,
    pub was_visible: bool,
    description: String,
}

impl CmdToggleVisibility {
    pub fn new(geometry_id: u64, was_visible: bool) -> Self {
        Self {
            geometry_id,
            was_visible,
            description: "Toggle Visibility".to_string(),
        }
    }
}

impl Command for CmdToggleVisibility {
    fn apply(&mut self, scene: &mut Scene) {
        if let Some(obj) = scene.geometries.iter_mut().find(|g| g.id == self.geometry_id) {
            obj.visible = !obj.visible;
            self.was_visible = obj.visible;
        }
    }
    
    fn undo(&mut self, scene: &mut Scene) {
        if let Some(obj) = scene.geometries.iter_mut().find(|g| g.id == self.geometry_id) {
            obj.visible = self.was_visible;
        }
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}