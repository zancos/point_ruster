//! UI module - egui-based UI panels

pub mod panels;
pub mod toolbar;
pub mod inspector;
pub mod scene_panel;

pub use panels::UI;
pub use toolbar::Toolbar;
pub use inspector::Inspector;
pub use scene_panel::ScenePanel;