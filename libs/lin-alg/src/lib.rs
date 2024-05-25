//! # Space, rotation and orientation related definitions
//!
//! This module contains various useful definitions that are used for operations on 3D space.
//! Most definitions are simply aliases to their implementation in `nalgebra` crate.

use nalgebra as na;

/// Generic, non-normalized quaternion.
pub type Quaternion = na::Quaternion<f32>;

/// Normalized quaternion, which can be used to represent orientation in 3D space.
pub type UnitQuaternion = na::Unit<Quaternion>;

/// Orientation in 3D space.
pub type Orientation = UnitQuaternion;

/// Point in 3D space.
pub type Point3 = na::Point3<f32>;
