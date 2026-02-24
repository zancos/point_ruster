//! Input handling - processes user input events

use winit::event::{MouseButton, Modifiers};

/// Input state for the application
#[derive(Debug, Clone)]
pub struct InputState {
    pub mouse_position: (f32, f32),
    pub mouse_delta: (f32, f32),
    pub mouse_pressed: MouseButton,
    pub mouse_held: Option<MouseButton>,
    pub modifiers: Modifiers,
    pub keys_pressed: std::collections::HashSet<winit::keyboard::Key>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            mouse_pressed: MouseButton::Left,
            mouse_held: None,
            modifiers: Modifiers::default(),
            keys_pressed: std::collections::HashSet::new(),
        }
    }

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: winit::keyboard::Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Check if Ctrl is held
    pub fn is_ctrl(&self) -> bool {
        self.modifiers.ctrl()
    }

    /// Check if Shift is held
    pub fn is_shift(&self) -> bool {
        self.modifiers.shift()
    }

    /// Check if Alt is held
    pub fn is_alt(&self) -> bool {
        self.modifiers.alt()
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}