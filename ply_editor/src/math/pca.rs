//! PCA (Principal Component Analysis) utilities

use glam::Vec3;

/// Compute the 3x3 covariance matrix of a set of points
pub fn compute_covariance(points: &[Vec3]) -> [[f32; 3]; 3] {
    if points.is_empty() {
        return [[0.0; 3]; 3];
    }

    // Compute centroid
    let centroid: Vec3 = points.iter().sum::<Vec3>() / points.len() as f32;

    // Compute covariance
    let mut cov = [[0.0f32; 3]; 3];

    for p in points {
        let d = *p - centroid;
        
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
    let n = points.len() as f32;
    for i in 0..3 {
        for j in 0..3 {
            cov[i][j] /= n;
        }
    }

    cov
}

/// PCA result
#[derive(Debug, Clone)]
pub struct PcaResult {
    /// Centroid of the points
    pub centroid: Vec3,
    /// Principal axes (eigenvectors), sorted by eigenvalue
    pub axes: [Vec3; 3],
    /// Eigenvalues
    pub eigenvalues: [f32; 3],
}

/// Perform PCA on a set of 3D points
/// 
/// Returns the principal axes and eigenvalues
pub fn pca_3d(points: &[Vec3]) -> Option<PcaResult> {
    if points.len() < 3 {
        return None;
    }

    let centroid: Vec3 = points.iter().sum::<Vec3>() / points.len() as f32;
    let cov = compute_covariance(points);

    // For a 3x3 symmetric matrix, we can compute eigenvalues analytically
    // or use a simple power iteration method
    
    // Compute eigenvalues using characteristic polynomial
    let a = cov[0][0];
    let b = cov[0][1];
    let c = cov[0][2];
    let d = cov[1][1];
    let e = cov[1][2];
    let f = cov[2][2];

    // Characteristic polynomial coefficients
    let p1 = a + d + f;
    let p2 = a * d + a * f + d * f - b * b - c * c - e * e;
    let p3 = a * d * f + 2.0 * b * c * e - a * e * e - d * c * c - f * b * b;

    // Solve for eigenvalues (simplified - using cubic formula would be better)
    // For now, use a simple iterative approach
    
    let mut eigenvalues = [0.0f32; 3];
    let mut eigenvectors = [Vec3::ZERO; 3];

    // Power iteration for largest eigenvalue
    let (ev1, vec1) = power_iteration(&cov, 50);
    eigenvalues[0] = ev1;
    eigenvectors[0] = vec1;

    // Deflation and second eigenvalue
    let cov2 = deflate(&cov, ev1, vec1);
    let (ev2, vec2) = power_iteration(&cov2, 50);
    eigenvalues[1] = ev2;
    eigenvectors[1] = vec2;

    // Third eigenvalue from trace
    eigenvalues[2] = p1 - ev1 - ev2;
    eigenvectors[2] = Vec3::cross(eigenvectors[0], eigenvectors[1]).normalize();

    // Sort by eigenvalue (descending)
    let mut pairs = [
        (eigenvalues[0], eigenvectors[0]),
        (eigenvalues[1], eigenvectors[1]),
        (eigenvalues[2], eigenvectors[2]),
    ];
    pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    eigenvalues = [pairs[0].0, pairs[1].0, pairs[2].0];
    let axes = [pairs[0].1.normalize(), pairs[1].1.normalize(), pairs[2].1.normalize()];

    Some(PcaResult {
        centroid,
        axes,
        eigenvalues,
    })
}

/// Power iteration to find the dominant eigenvalue and eigenvector
fn power_iteration(matrix: &[[f32; 3]; 3], iterations: usize) -> (f32, Vec3) {
    let mut v = Vec3::new(1.0, 0.5, 0.25).normalize();

    for _ in 0..iterations {
        // Matrix-vector multiplication
        let mut result = Vec3::ZERO;
        for i in 0..3 {
            result.x += matrix[0][i] * v[i];
            result.y += matrix[1][i] * v[i];
            result.z += matrix[2][i] * v[i];
        }

        let norm = result.length();
        if norm > 0.0001 {
            v = result / norm;
        }
    }

    // Compute eigenvalue (Rayleigh quotient)
    let mut av = Vec3::ZERO;
    for i in 0..3 {
        av.x += matrix[0][i] * v[i];
        av.y += matrix[1][i] * v[i];
        av.z += matrix[2][i] * v[i];
    }
    let eigenvalue = Vec3::dot(v, av);

    (eigenvalue, v)
}

/// Deflate a matrix by removing the contribution of an eigenvalue/eigenvector pair
fn deflate(matrix: &[[f32; 3]; 3], eigenvalue: f32, eigenvector: Vec3) -> [[f32; 3]; 3] {
    let mut result = [[0.0f32; 3]; 3];
    let ev = eigenvalue * eigenvector;

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[i][j] - eigenvector[i] * ev[j];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_covariance() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
        ];
        
        let cov = compute_covariance(&points);
        
        // Should have variance in X and Y, no covariance
        assert!(cov[0][0] > 0.0);
        assert!(cov[1][1] > 0.0);
    }
    
    #[test]
    fn test_pca() {
        // Points along X axis
        let points = vec![
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ];
        
        let result = pca_3d(&points).unwrap();
        
        // First axis should be close to X
        assert!(result.axes[0].x.abs() > 0.9);
    }
}