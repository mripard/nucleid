// Copyright 2020-2021, Cerno
// Licensed under the MIT License
// See the LICENSE file or <http://opensource.org/licenses/MIT>

#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![warn(clippy::multiple_crate_versions)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::use_self)]

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
mod raw;

pub use crate::buffer::Buffer;
pub use crate::buffer::Framebuffer;
pub use crate::buffer::Type as BufferType;
pub use crate::connector::Connector;
pub use crate::crtc::Crtc;
pub use crate::device::Device;
pub use crate::format::Format;
pub use crate::mode::Mode;
pub use crate::object::Object;
pub use crate::output::ConnectorUpdate;
pub use crate::output::ObjectUpdate;
pub use crate::output::Output;
pub use crate::output::PlaneUpdate;
pub use crate::output::Update;
pub use crate::plane::drm_plane_type as PlaneType;
pub use crate::plane::Plane;
pub use crate::property::Property;
pub use crate::raw::drm_connector_status as ConnectorStatus;
pub use crate::raw::drm_mode_connector_type as ConnectorType;
pub use crate::raw::drm_mode_type as ModeType;
