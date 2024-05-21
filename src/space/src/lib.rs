//! # Space, rotation and orientation related definitions
//!
//! This module contains various useful definitions that are used for operations on 3D space.
//! Most definitions are simply aliases to their implementation in `nalgebra` crate.

/// Generic, non-normalized quaternion.
pub type Quaternion = nalgebra::Quaternion<f64>;

/// Normalized quaternion, which can be used to represent orientation in 3D space.
pub type UnitQuaternion = nalgebra::Unit<Quaternion>;

/// Orientation in 3D space.
pub type Orientation = UnitQuaternion;

/// Point in 3D space.
pub type Point3 = nalgebra::Point3<f64>;
