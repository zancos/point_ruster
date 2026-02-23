//! Basis utilities - orthonormal basis creation

use glam::Vec3;

/// Create an orthonormal basis with the given normal
/// 
/// Returns (u, v, n) where n is the normalized input normal,
/// and u and v are orthonormal vectors in the plane perpendicular to n
pub fn make_basis(normal: Vec3) -> (Vec3, Vec3, Vec3) {
    let n = normal.normalize();
    
    // Find a vector not parallel to n
    let reference = if n.y.abs() < 0.99 { Vec3::Y } else { Vec3::X };
    
    // u = cross(n, reference), then normalize
    let u = Vec3::cross(n, reference).normalize();
    
    // v = cross(n, u)
    let v = Vec3::cross(n, u).normalize();
    
    (u, v, n)
}

/// Orthogonalize a set of vectors using Gram-Schmidt
/// 
/// Takes a slice of vectors and orthogonalizes them in place
pub fn orthogonalize(vectors: &mut [Vec3]) {
    if vectors.is_empty() {
        return;
    }

    for i in 0..vectors.len() {
        // Make this vector orthogonal to all previous ones
        for j in 0..i {
            let dot = Vec3::dot(vectors[i], vectors[j]);
            vectors[i] = vectors[i] - vectors[j] * dot;
        }
        
        // Normalize
        let len = vectors[i].length();
        if len > 0.0001 {
            vectors[i] = vectors[i] / len;
        }
    }
}

/// Project a point onto a plane defined by center and normal
pub fn project_to_plane(point: Vec3, plane_center: Vec3, plane_normal: Vec3) -> Vec3 {
    let n = plane_normal.normalize();
    let to_point = point - plane_center;
    let distance = Vec3::dot(to_point, n);
    point - n * distance
}

/// Compute the signed distance from a point to a plane
pub fn signed_distance_to_plane(point: Vec3, plane_center: Vec3, plane_normal: Vec3) -> f32 {
    let n = plane_normal.normalize();
    let to_point = point - plane_center;
    Vec3::dot(to_point, n)
}

/// Reflect a point across a plane
pub fn reflect_across_plane(point: Vec3, plane_center: Vec3, plane_normal: Vec3) -> Vec3 {
    let n = plane_normal.normalize();
    let to_point = point - plane_center;
    let distance = Vec3::dot(to_point, n);
    point - n * 2.0 * distance
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_make_basis() {
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let (u, v, n) = make_basis(normal);
        
        // Check orthogonality
        assert!(Vec3::dot(u, n).abs() < 0.001);
        assert!(Vec3::dot(v, n).abs() < 0.001);
        assert!(Vec3::dot(u, v).abs() < 0.001);
        
        // Check normalization
        assert!((u.length() - 1.0).abs() < 0.001);
        assert!((v.length() - 1.0).abs() < 0.001);
        assert!((n.length() - 1.0).abs() < 0.001);
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