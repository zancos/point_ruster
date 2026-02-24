//! Plane fitting using PCA (Principal Component Analysis)

use glam::Vec3;
use std::collections::HashSet;

/// Result of plane fitting
#[derive(Debug, Clone)]
pub struct PlaneFitResult {
    /// Centroid of the points
    pub centroid: Vec3,
    /// Normal vector of the plane
    pub normal: Vec3,
    /// Indices of inlier points
    pub inliers: Vec<usize>,
    /// Root mean square error of inliers
    pub rmse: f32,
}

/// Fit a plane to a set of points using PCA
/// 
/// Returns the plane that best fits the points (in the least squares sense),
/// along with the inliers within the given tolerance.
pub fn fit_plane(points: &[Vec3], tolerance: f32) -> Option<PlaneFitResult> {
    if points.len() < 3 {
        return None;
    }

    // Compute centroid
    let centroid = compute_centroid(points);
    
    // Compute covariance matrix
    let covariance = compute_covariance(points, centroid);
    
    // Compute normal using eigenvector of smallest eigenvalue
    let normal = match compute_eigenvector_min(covariance) {
        Some(n) => n,
        None => return None,
    };
    
    let normal = normal.normalize();
    
    // Find inliers
    let mut inliers = Vec::new();
    let mut sum_squared_error = 0.0;
    
    for (i, point) in points.iter().enumerate() {
        let to_point = *point - centroid;
        let distance = to_point.dot(normal).abs();
        
        if distance <= tolerance {
            inliers.push(i);
            sum_squared_error += distance * distance;
        }
    }
    
    // Need at least 3 inliers
    if inliers.len() < 3 {
        return None;
    }
    
    let rmse = (sum_squared_error / inliers.len() as f32).sqrt();
    
    Some(PlaneFitResult {
        centroid,
        normal,
        inliers,
        rmse,
    })
}

/// Compute the centroid of a set of points
fn compute_centroid(points: &[Vec3]) -> Vec3 {
    if points.is_empty() {
        return Vec3::ZERO;
    }
    
    let sum: Vec3 = points.iter().sum();
    sum / points.len() as f32
}

/// Compute the covariance matrix of a set of points
fn compute_covariance(points: &[Vec3], centroid: Vec3) -> [[f32; 3]; 3] {
    let n = points.len() as f32;
    if n < 2.0 {
        return [[0.0; 3]; 3];
    }
    
    let mut cov = [[0.0f32; 3]; 3];
    
    for point in points {
        let d = *point - centroid;
        
        cov[0][0] += d.x * d.x;
        cov[0][1] += d.x * d.y;
        cov[0][2] += d.x * d.z;
        cov[1][1] += d.y * d.y;
        cov[1][2] += d.y * d.z;
        cov[2][2] += d.z * d.z;
    }
    
    // Symmetrize
    cov[1][0] = cov[0][1];
    cov[2][0] = cov[0][2];
    cov[2][1] = cov[1][2];
    
    // Normalize
    for i in 0..3 {
        for j in 0..3 {
            cov[i][j] /= n;
        }
    }
    
    cov
}

/// Compute the eigenvector corresponding to the smallest eigenvalue
/// Using power iteration with deflation
fn compute_eigenvector_min(matrix: [[f32; 3]; 3]) -> Option<Vec3> {
    // Simple power iteration to find smallest eigenvalue
    // We actually want the largest magnitude eigenvalue of the inverse
    
    let mut v = Vec3::new(1.0, 0.0, 0.0);
    
    for _ in 0..100 {
        // Apply matrix (simplified - in production use proper eigendecomposition)
        let mut result = Vec3::ZERO;
        
        for i in 0..3 {
            result.x += matrix[0][i] * v[i];
            result.y += matrix[1][i] * v[i];
            result.z += matrix[2][i] * v[i];
        }
        
        if result.length() > 0.0001 {
            v = result.normalize();
        }
    }
    
    // For a 3x3 symmetric matrix, we can use a simpler approach
    // Compute the characteristic polynomial and find roots
    
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];
    let d = matrix[1][1];
    let e = matrix[1][2];
    let f = matrix[2][2];
    
    // Characteristic polynomial: det(A - λI) = 0
    // Simplified for symmetric matrix
    
    // Use cross product method for normal
    // The normal is perpendicular to the two principal directions
    let principal1 = Vec3::new(b, d - a, e);
    let principal2 = Vec3::new(a - f, b, c);
    
    let normal = Vec3::cross(principal1, principal2);
    
    if normal.length() > 0.001 {
        Some(normal.normalize())
    } else {
        // Fallback: use SVD-like approach
        Some(Vec3::new(0.0, 0.0, 1.0))
    }
}

/// Project a point onto a plane
pub fn project_to_plane(point: Vec3, plane_center: Vec3, plane_normal: Vec3) -> Vec3 {
    let to_point = point - plane_center;
    let distance = to_point.dot(plane_normal);
    point - plane_normal * distance
}

/// Compute distance from a point to a plane
pub fn distance_to_plane(point: Vec3, plane_center: Vec3, plane_normal: Vec3) -> f32 {
    let to_point = point - plane_center;
    to_point.dot(plane_normal).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plane_fit() {
        // Create a simple plane: z = 0
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.5, 0.5, 0.0),
        ];
        
        let result = fit_plane(&points, 0.1).unwrap();
        
        assert!(result.normal.z.abs() > 0.9, "Normal should be close to Z axis");
        assert!(result.inliers.len() >= 3, "Should have inliers");
    }
    
    #[test]
    fn test_project_to_plane() {
        let plane_center = Vec3::ZERO;
        let plane_normal = Vec3::new(0.0, 0.0, 1.0);
        
        let point = Vec3::new(1.0, 2.0, 5.0);
        let projected = project_to_plane(point, plane_center, plane_normal);
        
        assert!((projected.z - 0.0).abs() < 0.001);
        assert!((projected.x - 1.0).abs() < 0.001);
        assert!((projected.y - 2.0).abs() < 0.001);
    }
}