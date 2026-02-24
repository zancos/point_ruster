//! Primitives - helper for rendering geometric primitives

use glam::Vec3;
use crate::data::geometry::{Geometry, SurfaceMesh};

/// Helper for rendering geometric primitives
pub struct PrimitiveRenderer;

impl PrimitiveRenderer {
    /// Get vertices and indices for a geometry
    pub fn get_mesh(geometry: &Geometry) -> (Vec<Vec3>, Vec<u32>) {
        match geometry {
            Geometry::SurfaceMesh(mesh) => {
                (mesh.vertices.clone(), mesh.indices.clone())
            }
            Geometry::PlanePrimitive(plane) => {
                (plane.vertices(), plane.indices())
            }
            Geometry::Polyline(polyline) => {
                let mut vertices = polyline.points.clone();
                let indices: Vec<u32> = (0..vertices.len() as u32).collect();
                (vertices, indices)
            }
            Geometry::Sphere(sphere) => {
                Self::sphere_mesh(sphere.center, sphere.radius, 16, 16)
            }
            Geometry::Cylinder(cyl) => {
                Self::cylinder_mesh(cyl.center, cyl.axis, cyl.radius, cyl.height, 16)
            }
        }
    }

    /// Generate a sphere mesh
    pub fn sphere_mesh(center: Vec3, radius: f32, segments: u32, rings: u32) -> (Vec<Vec3>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for ring in 0..=rings {
            let phi = std::f32::consts::PI * (ring as f32 / rings as f32);
            let y = radius * phi.cos();
            let ring_radius = radius * phi.sin();

            for seg in 0..=segments {
                let theta = 2.0 * std::f32::consts::PI * (seg as f32 / segments as f32);
                let x = ring_radius * theta.cos();
                let z = ring_radius * theta.sin();

                vertices.push(center + Vec3::new(x, y, z));
            }
        }

        for ring in 0..rings {
            for seg in 0..segments {
                let current = ring * (segments + 1) + seg;
                let next = current + segments + 1;

                indices.push(current);
                indices.push(next);
                indices.push(current + 1);

                indices.push(current + 1);
                indices.push(next);
                indices.push(next + 1);
            }
        }

        (vertices, indices)
    }

    /// Generate a cylinder mesh
    pub fn cylinder_mesh(center: Vec3, axis: Vec3, radius: f32, height: f32, segments: u32) -> (Vec<Vec3>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create orthonormal basis
        let axis = axis.normalize();
        let any = if axis.y.abs() < 0.99 { Vec3::Y } else { Vec3::X };
        let right = Vec3::cross(axis, any).normalize();
        let up = Vec3::cross(right, axis).normalize();

        let half_height = height / 2.0;

        // Generate vertices
        for i in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * (i as f32 / segments as f32);
            let x = radius * theta.cos();
            let z = radius * theta.sin();

            let offset = right * x + up * z;

            // Bottom vertex
            vertices.push(center - axis * half_height + offset);
            // Top vertex
            vertices.push(center + axis * half_height + offset);
        }

        // Generate indices
        for i in 0..segments {
            let current = i * 2;
            let next = (i + 1) * 2;

            // Side faces
            indices.push(current);
            indices.push(next);
            indices.push(current + 1);

            indices.push(current + 1);
            indices.push(next);
            indices.push(next + 1);
        }

        (vertices, indices)
    }

    /// Get wireframe vertices for a geometry
    pub fn get_wireframe(geometry: &Geometry) -> Vec<Vec3> {
        match geometry {
            Geometry::SurfaceMesh(mesh) => {
                let mut lines = Vec::new();
                for tri in mesh.indices.chunks(3) {
                    if tri.len() == 3 {
                        let v0 = mesh.vertices[tri[0] as usize];
                        let v1 = mesh.vertices[tri[1] as usize];
                        let v2 = mesh.vertices[tri[2] as usize];
                        lines.push(v0); lines.push(v1);
                        lines.push(v1); lines.push(v2);
                        lines.push(v2); lines.push(v0);
                    }
                }
                lines
            }
            Geometry::PlanePrimitive(plane) => {
                let corners = plane.corners();
                vec![
                    corners[0], corners[1],
                    corners[1], corners[2],
                    corners[2], corners[3],
                    corners[3], corners[0],
                ]
            }
            Geometry::Polyline(polyline) => {
                polyline.points.clone()
            }
            Geometry::Sphere(_) | Geometry::Cylinder(_) => {
                Vec::new() // TODO: Generate wireframe
            }
        }
    }
}