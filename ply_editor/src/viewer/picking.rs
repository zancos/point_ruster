//! Picking - ray-based selection

use glam::{Vec3, Mat4};
use crate::data::PointCloud;
use crate::viewer::camera::Camera;

/// Picking result
#[derive(Debug, Clone)]
pub struct PickResult {
    /// Index of the picked point
    pub point_index: usize,
    /// Distance to the point
    pub distance: f32,
    /// Position of the picked point
    pub position: Vec3,
}

/// Picking system for selecting points
pub struct Picking {
    /// Maximum distance for picking
    pub max_pick_distance: f32,
    /// Downsample factor for large clouds
    pub downsample: usize,
}

impl Picking {
    pub fn new() -> Self {
        Self {
            max_pick_distance: 0.5,
            downsample: 1,
        }
    }

    /// Pick the nearest point to a screen position
    pub fn pick_point(
        &self,
        cloud: &PointCloud,
        camera: &Camera,
        screen_x: f32,
        screen_y: f32,
    ) -> Option<PickResult> {
        if cloud.is_empty() {
            return None;
        }

        let view_proj = camera.view_projection();
        let inv_view_proj = view_proj.inverse();
        
        // Create ray from screen position
        let ray = self.screen_to_ray(
            screen_x,
            screen_y,
            camera.aspect_ratio,
            camera.fov,
            &inv_view_proj,
        )?;
        
        // Find nearest point
        let mut nearest: Option<PickResult> = None;
        
        for (i, point) in cloud.points.iter().enumerate() {
            if self.downsample > 1 && i % self.downsample != 0 {
                continue;
            }
            
            // Distance from point to ray
            let to_point = *point - ray.origin;
            let proj_length = to_point.dot(ray.direction);
            
            if proj_length < 0.0 {
                continue; // Point is behind the camera
            }
            
            let closest_on_ray = ray.origin + ray.direction * proj_length;
            let dist = (*point - closest_on_ray).length();
            
            if dist < self.max_pick_distance {
                if nearest.is_none() || dist < nearest.as_ref().unwrap().distance {
                    nearest = Some(PickResult {
                        point_index: i,
                        distance: dist,
                        position: *point,
                    });
                }
            }
        }
        
        nearest
    }

    /// Perform box selection
    pub fn box_select(
        &self,
        cloud: &PointCloud,
        camera: &Camera,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
    ) -> Vec<usize> {
        if cloud.is_empty() {
            return Vec::new();
        }

        let view_proj = camera.view_projection();
        let mut selected = Vec::new();
        
        for (i, point) in cloud.points.iter().enumerate() {
            // Project point to screen space
            let clip_pos = view_proj.project_point3(*point);
            
            // Check if in box (normalized device coordinates)
            if clip_pos.x >= min_x && clip_pos.x <= max_x &&
               clip_pos.y >= min_y && clip_pos.y <= max_y &&
               clip_pos.z >= 0.0 && clip_pos.z <= 1.0 {
                selected.push(i);
            }
        }
        
        selected
    }

    /// Convert screen coordinates to ray
    fn screen_to_ray(
        &self,
        screen_x: f32,
        screen_y: f32,
        aspect: f32,
        fov: f32,
        inv_view_proj: &Mat4,
    ) -> Option<Ray> {
        // Convert to NDC
        let ndc_x = screen_x;
        let ndc_y = 1.0 - screen_y; // Flip Y
        
        // Near and far points in clip space
        let near_point = Vec3::new(ndc_x, ndc_y, 0.0);
        let far_point = Vec3::new(ndc_x, ndc_y, 1.0);
        
        // Transform to world space
        let near_world = inv_view_proj.project_point3(near_point);
        let far_world = inv_view_proj.project_point3(far_point);
        
        let direction = (far_world - near_world).normalize();
        
        Some(Ray {
            origin: near_world,
            direction,
        })
    }
}

impl Default for Picking {
    fn default() -> Self {
        Self::new()
    }
}

/// A ray in 3D space
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Get a point along the ray
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Distance from point to ray
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        let to_point = point - self.origin;
        let proj_length = to_point.dot(self.direction);
        let closest = self.at(proj_length.max(0.0));
        (point - closest).length()
    }
}