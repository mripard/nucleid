// Copyright 2020-2021, Cerno
// Licensed under the MIT License
// See the LICENSE file or <http://opensource.org/licenses/MIT>
//
//! # Direct Rendering Manager (DRM) abstraction crate
//!
//! This library contains code to interact with a DRM device.
//!
//! This library is a work in progress: expect missing features and breaking changes.
//!
//! # Hello World
//!
//! ```no_run
//! extern crate nucleid;
//!
//! use nucleid::Device;
//!
//! fn main() {
//!     let device = Device::new("/dev/dri/card0")
//!         .unwrap();
//! }
//! ```

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![allow(clippy::unreadable_literal)]

#[macro_use]
extern crate vmm_sys_util;

mod buffer;
mod connector;
mod crtc;
mod device;
mod encoder;
mod error;
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
pub use crate::connector::Status as ConnectorStatus;
pub use crate::connector::Type as ConnectorType;
pub use crate::crtc::Crtc;
pub use crate::device::Device;
pub use crate::error::Error;
pub use crate::error::Result;
pub use crate::format::Format;
pub use crate::mode::Mode;
pub use crate::output::ConnectorUpdate;
pub use crate::output::ObjectUpdate;
pub use crate::output::Output;
pub use crate::output::PlaneUpdate;
pub use crate::output::Update;
pub use crate::plane::Plane;
pub use crate::plane::Type as PlaneType;
pub use crate::property::Property;
