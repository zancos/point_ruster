//! Camera - handles camera modes and transformations

use glam::{Vec3, Mat4};
use winit::dpi::PhysicalSize;

/// Camera mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// Orbit mode (CAD-like)
    Orbit,
    /// First-person mode (FPS-like)
    FirstPerson,
}

/// Camera for 3D rendering
#[derive(Debug, Clone)]
pub struct Camera {
    /// Current mode
    pub mode: CameraMode,
    
    // Orbit parameters
    /// Target point for orbit
    pub orbit_target: Vec3,
    /// Distance from target
    pub orbit_distance: f32,
    /// Yaw angle (horizontal rotation)
    pub orbit_yaw: f32,
    /// Pitch angle (vertical rotation)
    pub orbit_pitch: f32,
    
    // FPS parameters
    /// Position in world space
    pub fps_position: Vec3,
    /// Yaw angle
    pub fps_yaw: f32,
    /// Pitch angle
    pub fps_pitch: f32,
    
    // Common
    /// Field of view
    pub fov: f32,
    /// Near plane
    pub near: f32,
    /// Far plane
    pub far: f32,
    /// Aspect ratio
    pub aspect_ratio: f32,
    
    // Movement
    /// Movement speed for FPS
    pub move_speed: f32,
    /// Mouse sensitivity
    pub mouse_sensitivity: f32,
}

impl Camera {
    /// Create a new camera
    pub fn new() -> Self {
        Self {
            mode: CameraMode::Orbit,
            orbit_target: Vec3::ZERO,
            orbit_distance: 10.0,
            orbit_yaw: 0.0,
            orbit_pitch: -0.3,
            fps_position: Vec3::new(0.0, 1.7, 5.0),
            fps_yaw: 0.0,
            fps_pitch: 0.0,
            fov: 60.0_f32.to_radians(),
            near: 0.01,
            far: 1000.0,
            aspect_ratio: 16.0 / 9.0,
            move_speed: 5.0,
            mouse_sensitivity: 0.002,
        }
    }

    /// Set the aspect ratio
    pub fn set_aspect(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height.max(1.0);
    }

    /// Set the aspect ratio from a PhysicalSize
    pub fn set_aspect_from_size(&mut self, size: PhysicalSize<u32>) {
        self.set_aspect(size.width as f32, size.height as f32);
    }

    /// Toggle between camera modes
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            CameraMode::Orbit => CameraMode::FirstPerson,
            CameraMode::FirstPerson => CameraMode::Orbit,
        };
    }

    /// Get the view matrix
    pub fn view_matrix(&self) -> Mat4 {
        match self.mode {
            CameraMode::Orbit => self.orbit_view_matrix(),
            CameraMode::FirstPerson => self.fps_view_matrix(),
        }
    }

    /// Get the projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    /// Get the view-projection matrix
    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Get the camera position
    pub fn position(&self) -> Vec3 {
        match self.mode {
            CameraMode::Orbit => self.orbit_position(),
            CameraMode::FirstPerson => self.fps_position,
        }
    }

    /// Orbit view matrix
    fn orbit_view_matrix(&self) -> Mat4 {
        let pos = self.orbit_position();
        Mat4::look_at_rh(pos, self.orbit_target, Vec3::Y)
    }

    /// FPS view matrix
    fn fps_view_matrix(&self) -> Mat4 {
        let forward = self.fps_forward();
        let right = self.fps_right();
        let up = Vec3::Y;
        
        Mat4::look_at_rh(
            self.fps_position,
            self.fps_position + forward,
            up
        )
    }

    /// Get orbit camera position
    fn orbit_position(&self) -> Vec3 {
        let x = self.orbit_distance * self.orbit_yaw.cos() * self.orbit_pitch.cos();
        let y = self.orbit_distance * self.orbit_pitch.sin();
        let z = self.orbit_distance * self.orbit_yaw.sin() * self.orbit_pitch.cos();
        
        self.orbit_target + Vec3::new(x, y, z)
    }

    /// Get forward vector for FPS
    fn fps_forward(&self) -> Vec3 {
        Vec3::new(
            self.fps_yaw.cos() * self.fps_pitch.cos(),
            self.fps_pitch.sin(),
            self.fps_yaw.sin() * self.fps_pitch.cos()
        ).normalize()
    }

    /// Get right vector for FPS
    fn fps_right(&self) -> Vec3 {
        Vec3::cross(self.fps_forward(), Vec3::Y).normalize()
    }

    // Orbit controls
    /// Orbit rotate (delta in radians)
    pub fn orbit_rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.orbit_yaw += delta_yaw;
        self.orbit_pitch = (self.orbit_pitch + delta_pitch).clamp(-1.5, 1.5);
    }

    /// Orbit pan
    pub fn orbit_pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = Vec3::new(self.orbit_yaw.cos(), 0.0, self.orbit_yaw.sin()).normalize();
        let up = Vec3::Y;
        
        let pan_speed = self.orbit_distance * 0.001;
        self.orbit_target += right * delta_x * pan_speed;
        self.orbit_target += up * delta_y * pan_speed;
    }

    /// Orbit zoom
    pub fn orbit_zoom(&mut self, delta: f32) {
        self.orbit_distance = (self.orbit_distance * (1.0 + delta * 0.1)).clamp(0.1, 1000.0);
    }

    // FPS controls
    /// FPS look (mouse delta)
    pub fn fps_look(&mut self, delta_x: f32, delta_y: f32) {
        self.fps_yaw += delta_x * self.mouse_sensitivity;
        self.fps_pitch = (self.fps_pitch - delta_y * self.mouse_sensitivity).clamp(-1.5, 1.5);
    }

    /// FPS move forward/backward
    pub fn fps_move_forward(&mut self, delta: f32) {
        self.fps_position += self.fps_forward() * delta * self.move_speed;
    }

    /// FPS move left/right
    pub fn fps_move_right(&mut self, delta: f32) {
        self.fps_position += self.fps_right() * delta * self.move_speed;
    }

    /// FPS move up/down
    pub fn fps_move_vertical(&mut self, delta: f32) {
        self.fps_position.y += delta * self.move_speed;
    }

    /// Reset camera to default position
    pub fn reset(&mut self) {
        self.orbit_target = Vec3::ZERO;
        self.orbit_distance = 10.0;
        self.orbit_yaw = 0.0;
        self.orbit_pitch = -0.3;
        self.fps_position = Vec3::new(0.0, 1.7, 5.0);
        self.fps_yaw = 0.0;
        self.fps_pitch = 0.0;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}