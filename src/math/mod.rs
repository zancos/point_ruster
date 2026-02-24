//! Math module - mathematical utilities

pub mod pca;
pub mod basis;

pub use pca::{pca_3d, compute_covariance};
pub use basis::{make_basis, orthogonalize};