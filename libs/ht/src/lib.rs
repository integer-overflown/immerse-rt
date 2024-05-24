//! # Head-tracking interface module.
//!
//! This module contains traits that are implemented by different platform APIs.
//! Any current or future implementation of head-tracking feature shall comply with the traits
//! described in this module.

use std::sync::mpsc::Receiver;

pub use irt_lin_alg::{Orientation, Quaternion, UnitQuaternion};

/// Unknown, unexpected or otherwise unclassified error.
///
/// These errors generally describe platform-specific errors that do not fall under a category
/// of "known" errors.
///
/// The implementation of this always shall always prefer to expand [ApiError] enum if applicable
/// rather than let implementation use unknown errors for reporting. This practice aims to improve
/// the clarity of error reporting by letting client code treat each error type specifically.
#[derive(thiserror::Error, Debug)]
#[error("unknown error occurred: {0}")]
pub struct UnknownError(String);

impl UnknownError {
    pub fn new(description: String) -> Self {
        Self(description)
    }
}

/// API-specific error.
///
/// The errors of this type generalize a common reasons under which the head-tracking functionality
/// might not be available. If these errors are ever reported or not, highly depend on the platform
/// or API specifics. For example, some platforms might not have a permissions concept.
///
/// It's generally advised for implementations to prefer reporting errors using this enum rather
/// than [UnknownError] API. This will result in clearer and more specific error messages in
/// the client applications.
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    /// API is not available and cannot be used with the current configuration.
    /// This may be caused by a number of reasons, including OS version mismatch,
    /// usage of conditionally supported functionality in the head-tracking implementation or
    /// a lack of eligible device in the user system.
    #[error("not available (no eligible device or unsupported configuration)")]
    NotAvailable,
    /// Access to motion data is denied due to permissions.
    /// This may occur if a user has previously denied the application request to motion data.
    /// This error is only reported on the systems that support permissions.
    #[error("access to motion data is denied (permission not granted)")]
    PermissionDenied,
}

/// Error that might occur when requesting motion data.
///
/// Split up in categories of "common" errors, which require specific errors, and custom errors,
/// which may include any other platform- or API-specific errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("API error")]
    Api(
        #[from]
        #[source]
        ApiError,
    ),
    #[error(transparent)]
    Other(#[from] UnknownError),
}

/// Head tracking API implementation
///
/// Any current or future implementation shall comply with this trait.
pub trait HeadTracker {
    /// Start receiving motion updates.
    ///
    /// This function may fail if the API wrapped by the implementation is not available or
    /// cannot be accessed.
    ///
    /// If the function completes successfully, it returns a receiver object that can be used to
    /// pull the orientation updates.
    fn start_motion_updates(&self) -> Result<Receiver<Orientation>, Error>;

    /// Stop receiving motion updates.
    ///
    /// This function destroys the sender object returned by [HeadTracker::start_motion_updates], which
    /// will unblock the receiving end waiting for the updates.
    ///
    /// The updates may always be resumed by calling [HeadTracker::start_motion_updates] again.
    ///
    /// Return nothing upon success or a generic error if an API-specific failure has prevented
    /// the updates from stopping.
    fn stop_motion_updates(&self) -> Result<(), UnknownError>;
}
