//! Scene - container for all scene data

use super::{PointCloud, Geometry, SceneObject, Selection};
use std::collections::HashMap;

/// The main scene containing all data
#[derive(Debug, Clone)]
pub struct Scene {
    /// The point cloud
    pub cloud: PointCloud,
    /// Geometry objects in the scene
    pub geometries: Vec<SceneObject>,
    /// Selection state
    pub selection: Selection,
    /// Settings
    pub settings: SceneSettings,
    /// Map from geometry ID to index in geometries vector
    geometry_index: HashMap<u64, usize>,
}

impl Scene {
    /// Create a new empty scene
    pub fn new() -> Self {
        Self {
            cloud: PointCloud::new(),
            geometries: Vec::new(),
            selection: Selection::new(),
            settings: SceneSettings::default(),
            geometry_index: HashMap::new(),
        }
    }

    /// Load a point cloud into the scene
    pub fn load_point_cloud(&mut self, cloud: PointCloud) {
        self.cloud = cloud;
        self.selection.clear();
    }

    /// Add geometry to the scene
    pub fn add_geometry(&mut self, object: SceneObject) {
        let id = object.id;
        self.geometry_index.insert(id, self.geometries.len());
        self.geometries.push(object);
    }

    /// Remove geometry from the scene by ID
    pub fn remove_geometry(&mut self, id: u64) {
        if let Some(index) = self.geometry_index.remove(&id) {
            self.geometries.remove(index);
            // Update indices
            for (i, obj) in self.geometries.iter().enumerate() {
                self.geometry_index.insert(obj.id, i);
            }
        }
    }

    /// Get geometry by ID
    pub fn get_geometry(&self, id: u64) -> Option<&SceneObject> {
        self.geometries.iter().find(|g| g.id == id)
    }

    /// Get mutable geometry by ID
    pub fn get_geometry_mut(&mut self, id: u64) -> Option<&mut SceneObject> {
        self.geometries.iter_mut().find(|g| g.id == id)
    }

    /// Get visible geometries
    pub fn visible_geometries(&self) -> impl Iterator<Item = &SceneObject> {
        self.geometries.iter().filter(|g| g.visible)
    }

    /// Clear all geometries
    pub fn clear_geometries(&mut self) {
        self.geometries.clear();
        self.geometry_index.clear();
    }

    /// Get the number of geometries
    pub fn geometry_count(&self) -> usize {
        self.geometries.len()
    }

    /// Check if the scene has a point cloud
    pub fn has_cloud(&self) -> bool {
        !self.cloud.is_empty()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

/// Scene settings
#[derive(Debug, Clone)]
pub struct SceneSettings {
    /// Point size for rendering
    pub point_size: f32,
    /// Background color (RGB)
    pub background_color: [f32; 3],
    /// Show point cloud
    pub show_points: bool,
    /// Show geometries
    pub show_geometries: bool,
    /// Selection color
    pub selection_color: [f32; 3],
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            point_size: 2.0,
            background_color: [0.1, 0.1, 0.15],
            show_points: true,
            show_geometries: true,
            selection_color: [1.0, 0.8, 0.0],
        }
    }
}