//! Selection state for points and geometries

use std::collections::HashSet;

/// Selection state for the scene
#[derive(Debug, Clone, Default)]
pub struct Selection {
    /// Selected point indices
    pub points: HashSet<usize>,
    /// Selected geometry IDs
    pub geometries: HashSet<u64>,
}

impl Selection {
    /// Create a new empty selection
    pub fn new() -> Self {
        Self {
            points: HashSet::new(),
            geometries: HashSet::new(),
        }
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.points.clear();
        self.geometries.clear();
    }

    /// Clear point selection
    pub fn clear_points(&mut self) {
        self.points.clear();
    }

    /// Clear geometry selection
    pub fn clear_geometries(&mut self) {
        self.geometries.clear();
    }

    /// Add a point to selection
    pub fn add_point(&mut self, index: usize) {
        self.points.insert(index);
    }

    /// Add multiple points to selection
    pub fn add_points(&mut self, indices: impl IntoIterator<Item = usize>) {
        for i in indices {
            self.points.insert(i);
        }
    }

    /// Remove a point from selection
    pub fn remove_point(&mut self, index: usize) {
        self.points.remove(&index);
    }

    /// Check if a point is selected
    pub fn is_point_selected(&self, index: usize) -> bool {
        self.points.contains(&index)
    }

    /// Add a geometry to selection
    pub fn add_geometry(&mut self, id: u64) {
        self.geometries.insert(id);
    }

    /// Remove a geometry from selection
    pub fn remove_geometry(&mut self, id: u64) {
        self.geometries.remove(&id);
    }

    /// Check if a geometry is selected
    pub fn is_geometry_selected(&self, id: u64) -> bool {
        self.geometries.contains(&id)
    }

    /// Set the point selection (replaces existing)
    pub fn set_points(&mut self, indices: Vec<usize>) {
        self.points.clear();
        self.points.extend(indices);
    }

    /// Get the number of selected points
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// Get the number of selected geometries
    pub fn geometry_count(&self) -> usize {
        self.geometries.len()
    }

    /// Get all selected point indices as a vector
    pub fn selected_points(&self) -> Vec<usize> {
        self.points.iter().copied().collect()
    }

    /// Get all selected geometry IDs as a vector
    pub fn selected_geometries(&self) -> Vec<u64> {
        self.geometries.iter().copied().collect()
    }

    /// Toggle point selection
    pub fn toggle_point(&mut self, index: usize) {
        if self.points.contains(&index) {
            self.points.remove(&index);
        } else {
            self.points.insert(index);
        }
    }

    /// Toggle geometry selection
    pub fn toggle_geometry(&mut self, id: u64) {
        if self.geometries.contains(&id) {
            self.geometries.remove(&id);
        } else {
            self.geometries.insert(id);
        }
    }
}