//! Point cloud data structure

use glam::Vec3;
use std::ops::Index;

/// A point cloud with positions and optional colors/normals
#[derive(Debug, Clone)]
pub struct PointCloud {
    /// Point positions
    pub points: Vec<Vec3>,
    /// Point colors (RGB, 0-255), same length as points if present
    pub colors: Option<Vec<[u8; 3]>>,
    /// Point normals (optional)
    pub normals: Option<Vec<Vec3>>,
    /// Bounding box min
    pub bbox_min: Vec3,
    /// Bounding box max
    pub bbox_max: Vec3,
}

impl PointCloud {
    /// Create a new empty point cloud
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            colors: None,
            normals: None,
            bbox_min: Vec3::ZERO,
            bbox_max: Vec3::ZERO,
        }
    }

    /// Create a point cloud from points
    pub fn from_points(points: Vec<Vec3>) -> Self {
        let (bbox_min, bbox_max) = Self::compute_bbox(&points);
        Self {
            points,
            colors: None,
            normals: None,
            bbox_min,
            bbox_max,
        }
    }

    /// Create a point cloud from points with colors
    pub fn from_points_with_colors(points: Vec<Vec3>, colors: Vec<[u8; 3]>) -> Self {
        let (bbox_min, bbox_max) = Self::compute_bbox(&points);
        Self {
            points,
            colors: Some(colors),
            normals: None,
            bbox_min,
            bbox_max,
        }
    }

    /// Get the number of points
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if the point cloud is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get the center of the bounding box
    pub fn center(&self) -> Vec3 {
        (self.bbox_min + self.bbox_max) * 0.5
    }

    /// Get the size of the bounding box
    pub fn size(&self) -> Vec3 {
        self.bbox_max - self.bbox_min
    }

    /// Compute bounding box for a set of points
    fn compute_bbox(points: &[Vec3]) -> (Vec3, Vec3) {
        if points.is_empty() {
            return (Vec3::ZERO, Vec3::ZERO);
        }
        
        let mut min = points[0];
        let mut max = points[0];
        
        for p in points.iter().skip(1) {
            min = min.min(*p);
            max = max.max(*p);
        }
        
        (min, max)
    }

    /// Get a point by index
    pub fn get(&self, index: usize) -> Option<Vec3> {
        self.points.get(index).copied()
    }

    /// Get color by index
    pub fn get_color(&self, index: usize) -> Option<[u8; 3]> {
        self.colors.as_ref().and_then(|c| c.get(index).copied())
    }

    /// Apply a transformation to all points
    pub fn transform(&mut self, matrix: glam::Mat4) {
        for p in &mut self.points {
            *p = matrix.transform_point3(*p);
        }
        // Recompute bounding box
        let (min, max) = Self::compute_bbox(&self.points);
        self.bbox_min = min;
        self.bbox_max = max;
    }
}

impl Default for PointCloud {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for PointCloud {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}