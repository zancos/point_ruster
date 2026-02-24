//! Triangulation of points on a plane

use glam::Vec3;
use delaunator::Triangulation;
use std::collections::HashMap;

/// Triangulate points projected onto a plane
/// 
/// Takes 3D points that lie on a plane and returns triangle indices
pub fn triangulate(points: &[Vec3], plane_center: Vec3, plane_normal: Vec3) -> Option<Vec<u32>> {
    if points.len() < 3 {
        return None;
    }

    // Build orthonormal basis on the plane
    let u = if plane_normal.abs().y < 0.99 {
        Vec3::cross(plane_normal, Vec3::Y).normalize()
    } else {
        Vec3::cross(plane_normal, Vec3::X).normalize()
    };
    let v = Vec3::cross(plane_normal, u).normalize();

    // Project points to 2D
    let mut points_2d: Vec<(f64, f64)> = Vec::with_capacity(points.len());
    let mut index_map: HashMap<usize, usize> = HashMap::new();

    for (i, point) in points.iter().enumerate() {
        let local = *point - plane_center;
        let x = local.dot(u) as f64;
        let y = local.dot(v) as f64;
        points_2d.push((x, y));
        index_map.insert(i, i);
    }

    // Perform Delaunay triangulation
    let delaunay = Triangulation::new(&points_2d);

    // Extract triangle indices
    let mut indices = Vec::new();
    
    for i in 0..delaunay.triangles.len() {
        let t = delaunay.triangles[i];
        // Only include triangles that are valid (not on convex hull boundary if desired)
        // For now, include all triangles
        indices.push(t as u32);
    }

    if indices.is_empty() {
        None
    } else {
        Some(indices)
    }
}

/// Create a triangulated surface mesh from points on a plane
pub fn create_surface_mesh(
    points: &[Vec3],
    plane_center: Vec3,
    plane_normal: Vec3,
) -> Option<(Vec<Vec3>, Vec<u32>)> {
    let indices = triangulate(points, plane_center, plane_normal)?;
    
    // The indices refer to the input points directly
    let vertices = points.to_vec();
    
    Some((vertices, indices))
}

/// Compute the 2D bounding box of points on a plane
pub fn compute_2d_bbox(
    points: &[Vec3],
    plane_center: Vec3,
    plane_normal: Vec3,
) -> Option<(f32, f32, f32, f32)> {
    if points.is_empty() {
        return None;
    }

    let u = if plane_normal.abs().y < 0.99 {
        Vec3::cross(plane_normal, Vec3::Y).normalize()
    } else {
        Vec3::cross(plane_normal, Vec3::X).normalize()
    };
    let v = Vec3::cross(plane_normal, u).normalize();

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for point in points {
        let local = *point - plane_center;
        let x = local.dot(u);
        let y = local.dot(v);

        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    Some((min_x, max_x, min_y, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_triangulate() {
        // Create points on the XY plane (z = 0)
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.5, 0.5, 0.0),
        ];
        
        let result = triangulate(&points, Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        
        assert!(result.is_some());
        let indices = result.unwrap();
        
        // Should have triangles (at least 1)
        assert!(indices.len() >= 3);
    }
    
    #[test]
    fn test_2d_bbox() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        
        let bbox = compute_2d_bbox(&points, Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
        
        assert!(bbox.is_some());
        let (min_x, max_x, min_y, max_y) = bbox.unwrap();
        
        assert!((min_x - 0.0).abs() < 0.001);
        assert!((max_x - 1.0).abs() < 0.001);
        assert!((min_y - 0.0).abs() < 0.001);
        assert!((max_y - 1.0).abs() < 0.001);
    }
}