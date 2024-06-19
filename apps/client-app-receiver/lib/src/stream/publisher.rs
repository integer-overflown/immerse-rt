use std::error::Error;

use gst::prelude::*;
use tracing::{debug, error, warn};

pub struct StreamController {
    pipeline: gst::Element,
}

#[derive(thiserror::Error, Debug)]
pub enum SetupError {
    #[error("failed to create pipeline")]
    FailedToCreatePipeline,
}

pub fn create(token: &str, file_path: &str) -> Result<StreamController, SetupError> {
    debug!("Publishing: {file_path}");

    let desc = format!(
        "\
        livekitwebrtcsink name=sink signaller::auth-token=\"{}\" video-caps=\"video/x-vp8\" audio-caps=\"audio/x-opus\" \
        uridecodebin3 uri=\"{}\" name=bin \
        bin. \
        ! queue \
        ! videoconvert \
        ! sink. \
        bin. \
        ! audioconvert \
        ! audioresample \
        ! sink. \
        ",
        token, file_path
    );

    debug!("pipeline: {desc}");

    let pipeline = gst::parse::launch(&desc).map_err(|e| {
        error!("Failed to parse pipeline: {e}");
        SetupError::FailedToCreatePipeline
    })?;

    Ok(StreamController { pipeline })
}

impl StreamController {
    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        self.pipeline.set_state(gst::State::Playing)?;
        Ok(())
    }
}

impl Drop for StreamController {
    fn drop(&mut self) {
        debug!("Destroying");

        if let Err(e) = self.pipeline.set_state(gst::State::Null) {
            warn!("Failed to shut the pipeline down: {e}");
        }
    }
}
