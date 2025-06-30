// Copyright 2020-2021, Cerno
// Licensed under the MIT License
// See the LICENSE file or <http://opensource.org/licenses/MIT>

#![doc = include_str!("../README.md")]
#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]

mod buffer;
mod connector;
mod crtc;
mod device;
mod encoder;
mod format;
mod mode;
mod object;
mod output;
mod plane;
mod property;
pub mod raw;

pub use crate::{
    buffer::{Buffer, Framebuffer, Type as BufferType},
    connector::Connector,
    crtc::Crtc,
    device::Device,
    encoder::Encoder,
    format::Format,
    mode::Mode,
    object::Object,
    output::{ConnectorUpdate, ObjectUpdate, Output, PlaneUpdate, Update},
    plane::{Plane, PlaneType},
    property::Property,
    raw::{
        drm_connector_status as ConnectorStatus, drm_mode_connector_type as ConnectorType,
        drm_mode_encoder_type, drm_mode_type as ModeType,
    },
};
