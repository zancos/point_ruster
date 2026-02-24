//! Geometry types for the scene

use glam::Vec3;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique ID generator for scene objects
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Geometry types supported in the scene
#[derive(Debug, Clone)]
pub enum Geometry {
    /// A triangulated surface mesh
    SurfaceMesh(SurfaceMesh),
    /// A plane primitive (quad)
    PlanePrimitive(PlanePrimitive),
    /// A polyline
    Polyline(Polyline),
    /// A sphere
    Sphere(Sphere),
    /// A cylinder
    Cylinder(Cylinder),
}

/// A triangulated surface mesh
#[derive(Debug, Clone)]
pub struct SurfaceMesh {
    /// Vertices
    pub vertices: Vec<Vec3>,
    /// Triangle indices (3 per triangle)
    pub indices: Vec<u32>,
    /// Vertex colors (optional)
    pub colors: Option<Vec<[u8; 3]>>,
}

impl SurfaceMesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            colors: None,
        }
    }

    pub fn with_colors(mut self, colors: Vec<[u8; 3]>) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Get the number of triangles
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}

/// A plane primitive (quad)
#[derive(Debug, Clone)]
pub struct PlanePrimitive {
    /// Center of the plane
    pub center: Vec3,
    /// Normal vector
    pub normal: Vec3,
    /// Width (along U axis)
    pub width: f32,
    /// Height (along V axis)
    pub height: f32,
    /// U axis (in-plane, perpendicular to normal)
    pub u_axis: Vec3,
    /// V axis (in-plane, perpendicular to normal and U)
    pub v_axis: Vec3,
}

impl PlanePrimitive {
    pub fn new(center: Vec3, normal: Vec3, width: f32, height: f32) -> Self {
        // Compute orthonormal basis
        let normal = normal.normalize();
        let u_axis = if normal.abs().y < 0.99 {
            Vec3::cross(normal, Vec3::Y).normalize()
        } else {
            Vec3::cross(normal, Vec3::X).normalize()
        };
        let v_axis = Vec3::cross(normal, u_axis).normalize();

        Self {
            center,
            normal,
            width,
            height,
            u_axis,
            v_axis,
        }
    }

    /// Get the 4 corners of the plane
    pub fn corners(&self) -> [Vec3; 4] {
        let hw = self.width / 2.0;
        let hh = self.height / 2.0;
        
        [
            self.center - self.u_axis * hw - self.v_axis * hh,
            self.center + self.u_axis * hw - self.v_axis * hh,
            self.center + self.u_axis * hw + self.v_axis * hh,
            self.center - self.u_axis * hw + self.v_axis * hh,
        ]
    }

    /// Get vertices for rendering as a quad
    pub fn vertices(&self) -> Vec<Vec3> {
        let corners = self.corners();
        vec![
            corners[0], corners[1], corners[2],
            corners[0], corners[2], corners[3],
        ]
    }

    /// Get indices for rendering as triangles
    pub fn indices(&self) -> Vec<u32> {
        vec![0, 1, 2, 0, 2, 3]
    }
}

/// A polyline
#[derive(Debug, Clone)]
pub struct Polyline {
    /// Points along the polyline
    pub points: Vec<Vec3>,
}

impl Polyline {
    pub fn new(points: Vec<Vec3>) -> Self {
        Self { points }
    }
}

/// A sphere
#[derive(Debug, Clone)]
pub struct Sphere {
    /// Center of the sphere
    pub center: Vec3,
    /// Radius
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

/// A cylinder
#[derive(Debug, Clone)]
pub struct Cylinder {
    /// Center of the cylinder
    pub center: Vec3,
    /// Axis direction
    pub axis: Vec3,
    /// Radius
    pub radius: f32,
    /// Height
    pub height: f32,
}

impl Cylinder {
    pub fn new(center: Vec3, axis: Vec3, radius: f32, height: f32) -> Self {
        Self {
            center,
            axis: axis.normalize(),
            radius,
            height,
        }
    }
}

/// A scene object with geometry
#[derive(Debug, Clone)]
pub struct SceneObject {
    /// Unique ID
    pub id: u64,
    /// Name for display
    pub name: String,
    /// The geometry
    pub geometry: Geometry,
    /// Whether the object is visible
    pub visible: bool,
}

impl SceneObject {
    pub fn new(geometry: Geometry) -> Self {
        Self {
            id: next_id(),
            name: format!("Object_{}", next_id()),
            geometry,
            visible: true,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

impl fmt::Display for Geometry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Geometry::SurfaceMesh(_) => write!(f, "SurfaceMesh"),
            Geometry::PlanePrimitive(_) => write!(f, "PlanePrimitive"),
            Geometry::Polyline(_) => write!(f, "Polyline"),
            Geometry::Sphere(_) => write!(f, "Sphere"),
            Geometry::Cylinder(_) => write!(f, "Cylinder"),
        }
    }
}