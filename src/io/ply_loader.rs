//! PLY loader - load PLY files

use crate::data::PointCloud;
use glam::Vec3;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlyError {
    #[error("Failed to open file: {0}")]
    FileOpen(#[from] std::io::Error),
    
    #[error("Invalid PLY format: {0}")]
    InvalidFormat(String),
    
    #[error("Unsupported PLY format")]
    UnsupportedFormat,
    
    #[error("No vertices found")]
    NoVertices,
}

/// Load a PLY file
pub fn load_ply(path: &Path) -> Result<PointCloud, PlyError> {
    let content = std::fs::read_to_string(path)?;
    
    // Check header
    let mut lines = content.lines();
    let header = lines.next().ok_or_else(|| PlyError::InvalidFormat("Empty file".to_string()))?;
    
    if !header.trim().starts_with("ply") {
        return Err(PlyError::InvalidFormat("Not a PLY file".to_string()));
    }
    
    // Parse header to find vertex count and properties
    let mut vertex_count = 0;
    let mut has_colors = false;
    let mut format = "ascii";
    
    for line in lines.by_ref() {
        let line = line.trim();
        
        if line == "end_header" {
            break;
        }
        
        if line.starts_with("element vertex") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                vertex_count = parts[2].parse().unwrap_or(0);
            }
        } else if line.starts_with("format") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                format = parts[1];
            }
        } else if line.contains("red") || line.contains("green") || line.contains("blue") {
            has_colors = true;
        }
    }
    
    if vertex_count == 0 {
        return Err(PlyError::NoVertices);
    }
    
    // Parse based on format
    match format {
        "ascii" => load_ascii(&lines.collect::<Vec<_>>(), vertex_count, has_colors),
        "binary_little_endian" | "binary_big_endian" => {
            // For binary, we'd need to read the raw bytes
            // For now, return an error
            Err(PlyError::UnsupportedFormat)
        }
        _ => Err(PlyError::UnsupportedFormat),
    }
}

/// Load ASCII PLY format
fn load_ascii(lines: &[&str], vertex_count: usize, has_colors: bool) -> Result<PointCloud, PlyError> {
    let mut points = Vec::with_capacity(vertex_count);
    let mut colors = if has_colors { Some(Vec::with_capacity(vertex_count)) } else { None };
    
    for line in lines.iter().take(vertex_count) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        // Parse position (x, y, z)
        if parts.len() >= 3 {
            let x: f32 = parts[0].parse().unwrap_or(0.0);
            let y: f32 = parts[1].parse().unwrap_or(0.0);
            let z: f32 = parts[2].parse().unwrap_or(0.0);
            
            points.push(Vec3::new(x, y, z));
            
            // Parse colors if available (r, g, b)
            if has_colors && parts.len() >= 6 {
                let r: u8 = parts[3].parse().unwrap_or(255);
                let g: u8 = parts[4].parse().unwrap_or(255);
                let b: u8 = parts[5].parse().unwrap_or(255);
                
                if let Some(ref mut c) = colors {
                    c.push([r, g, b]);
                }
            }
        }
    }
    
    if points.is_empty() {
        return Err(PlyError::NoVertices);
    }
    
    let cloud = if let Some(c) = colors {
        PointCloud::from_points_with_colors(points, c)
    } else {
        PointCloud::from_points(points)
    };
    
    Ok(cloud)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_load_ply_ascii() {
        // Create a temporary PLY file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "ply").unwrap();
        writeln!(file, "format ascii 1.0").unwrap();
        writeln!(file, "element vertex 3").unwrap();
        writeln!(file, "property float x").unwrap();
        writeln!(file, "property float y").unwrap();
        writeln!(file, "property float z").unwrap();
        writeln!(file, "property uchar red").unwrap();
        writeln!(file, "property uchar green").unwrap();
        writeln!(file, "property uchar blue").unwrap();
        writeln!(file, "end_header").unwrap();
        writeln!(file, "0 0 0 255 0 0").unwrap();
        writeln!(file, "1 0 0 0 255 0").unwrap();
        writeln!(file, "0 1 0 0 0 255").unwrap();
        file.flush().unwrap();
        
        let cloud = load_ply(file.path()).unwrap();
        
        assert_eq!(cloud.len(), 3);
        assert!(cloud.colors.is_some());
    }
}