//! OBJ exporter - export geometries to OBJ format

use crate::data::geometry::{Geometry, SurfaceMesh};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObjError {
    #[error("Failed to create file: {0}")]
    FileCreate(#[from] std::io::Error),
    
    #[error("No geometry to export")]
    NoGeometry,
    
    #[error("Unsupported geometry type for OBJ export")]
    UnsupportedGeometry,
}

/// Export a geometry to OBJ file
pub fn export_obj(path: &Path, geometry: &Geometry) -> Result<(), ObjError> {
    let mut file = File::create(path)?;
    
    match geometry {
        Geometry::SurfaceMesh(mesh) => export_surface_mesh(&mut file, mesh)?,
        Geometry::PlanePrimitive(plane) => export_plane(&mut file, plane)?,
        _ => return Err(ObjError::UnsupportedGeometry),
    }
    
    Ok(())
}

/// Export a surface mesh to OBJ
fn export_surface_mesh(file: &mut File, mesh: &SurfaceMesh) -> Result<(), ObjError> {
    // Write vertices
    for v in &mesh.vertices {
        writeln!(file, "v {} {} {}", v.x, v.y, v.z)?;
    }
    
    // Write vertex colors if available
    if let Some(colors) = &mesh.colors {
        for c in colors {
            writeln!(file, "vc {} {} {}", c[0], c[1], c[2])?;
        }
    }
    
    // Write faces (OBJ uses 1-based indexing)
    for tri in mesh.indices.chunks(3) {
        if tri.len() == 3 {
            writeln!(file, "f {} {} {}", tri[0] + 1, tri[1] + 1, tri[2] + 1)?;
        }
    }
    
    Ok(())
}

/// Export a plane primitive to OBJ
fn export_plane(file: &mut File, plane: &crate::data::geometry::PlanePrimitive) -> Result<(), ObjError> {
    let corners = plane.corners();
    
    // Write vertices
    for c in &corners {
        writeln!(file, "v {} {} {}", c.x, c.y, c.z)?;
    }
    
    // Write faces (two triangles for the quad)
    writeln!(file, "f 1 2 3")?;
    writeln!(file, "f 1 3 4")?;
    
    Ok(())
}

/// Export all geometries in a scene to OBJ
pub fn export_scene_objs(path: &Path, geometries: &[crate::data::SceneObject]) -> Result<(), ObjError> {
    if geometries.is_empty() {
        return Err(ObjError::NoGeometry);
    }
    
    let mut file = File::create(path)?;
    
    // Write header
    writeln!(file, "# Exported from PLY Editor")?;
    writeln!(file, "")?;
    
    let mut vertex_offset = 1u32;
    
    for obj in geometries {
        writeln!(file, "# Object: {}", obj.name)?;
        
        match &obj.geometry {
            Geometry::SurfaceMesh(mesh) => {
                // Write vertices
                for v in &mesh.vertices {
                    writeln!(file, "v {} {} {}", v.x, v.y, v.z)?;
                }
                
                // Write faces with offset
                for tri in mesh.indices.chunks(3) {
                    if tri.len() == 3 {
                        writeln!(file, "f {} {} {}", 
                            tri[0] + vertex_offset, 
                            tri[1] + vertex_offset, 
                            tri[2] + vertex_offset
                        )?;
                    }
                }
                
                vertex_offset += mesh.vertices.len() as u32;
            }
            Geometry::PlanePrimitive(plane) => {
                let corners = plane.corners();
                for c in &corners {
                    writeln!(file, "v {} {} {}", c.x, c.y, c.z)?;
                }
                writeln!(file, "f {} {} {}", 
                    vertex_offset, 
                    vertex_offset + 1, 
                    vertex_offset + 2
                )?;
                writeln!(file, "f {} {} {}", 
                    vertex_offset, 
                    vertex_offset + 2, 
                    vertex_offset + 3
                )?;
                vertex_offset += 4;
            }
            _ => {} // Skip unsupported types
        }
        
        writeln!(file, "")?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::geometry::PlanePrimitive;
    use glam::Vec3;
    
    #[test]
    fn test_export_plane() {
        let plane = PlanePrimitive::new(
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, 1.0),
            2.0,
            2.0
        );
        
        let mut file = File::create("test_plane.obj").unwrap();
        export_plane(&mut file, &plane).unwrap();
        
        // Clean up
        std::fs::remove_file("test_plane.obj").ok();
    }
}