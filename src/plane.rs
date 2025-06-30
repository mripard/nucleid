extern crate alloc;

use alloc::rc::{Rc, Weak};
use core::{
    cell::RefCell,
    convert::{TryFrom, TryInto as _},
    fmt,
};
use std::io;

use facet_derive::Facet;
use facet_enum_repr::FacetEnumRepr;

use crate::{
    Device, Format,
    device::Inner,
    object::Object,
    raw::{drm_mode_get_plane, drm_mode_object_type},
};

/// The [Plane] types
#[derive(Debug, Eq, Facet, FacetEnumRepr, PartialEq)]
#[repr(u32)]
pub enum PlaneType {
    /// The [Plane] is an overlay, aka a sprite. Any plane that is neither a primary nor a cursor
    /// plane
    Overlay = 0,

    /// The main [Plane] the [CRTC](crate::Crtc) is acting upon during modesetting.
    Primary,

    /// The [Plane] is a cursor plane
    Cursor,
}

/// A representation of a image source sent to the CRTC
///
/// A Plane represents an image that will be blended by the CRTC during the scanout.
#[derive(Debug)]
pub struct Plane {
    dev: Weak<RefCell<Inner>>,
    id: u32,
    possible_crtcs: u32,
    formats: Vec<Format>,
}

impl Plane {
    pub(crate) fn new(device: &Device, id: u32) -> io::Result<Self> {
        let mut formats = Vec::new();
        let raw_plane = drm_mode_get_plane(device, id, Some(&mut formats))?;
        let mut plane = Self {
            dev: Rc::downgrade(&device.inner),
            id,
            possible_crtcs: raw_plane.possible_crtcs,
            formats: Vec::new(),
        };

        for raw_fmt in formats {
            if let Ok(fmt) = Format::try_from(raw_fmt) {
                plane.formats.push(fmt);
            }
        }

        Ok(plane)
    }

    pub(crate) const fn possible_crtcs(&self) -> u32 {
        self.possible_crtcs
    }

    /// Returns an Iterator over the [Formats](Format) supported by this plane
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{Device, Format};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let plane = device.planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn formats(&self) -> Formats<'_> {
        Formats {
            iter: self.formats.iter(),
        }
    }

    /// Returns the [Plane] type
    ///
    /// # Panics
    ///
    /// If the properties retrieval ioctl fails, or if the plane type property isn't found.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{Device, Format, PlaneType};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let plane = device.planes()
    ///     .into_iter()
    ///     .find(|plane| plane.plane_type().unwrap() == PlaneType::Primary)
    ///     .unwrap();
    /// ```
    pub fn plane_type(&self) -> io::Result<PlaneType> {
        let type_prop = self
            .properties()
            .map_err(|_e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "The kernel guarantees the string is null-terminated.",
                )
            })?
            .into_iter()
            .find(|prop| prop.name() == "type")
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Plane Missing a type property",
            ))?;

        // NOTE: the plane type returned by the kernel is an enum between 0 and 2. If we have
        // something that underflows or overflows an u32, we have a serious issue.
        let val: u32 = type_prop.value().try_into().map_err(|_e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected Value returned by the kernel.",
            )
        })?;

        PlaneType::try_from(val).map_err(|_e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected Plane Type returned by the kernel.",
            )
        })
    }
}

impl Object for Plane {
    fn device(&self) -> Device {
        self.dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into()
    }

    fn object_id(&self) -> u32 {
        self.id
    }

    fn object_type(&self) -> drm_mode_object_type {
        drm_mode_object_type::Plane
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("plane-{}", self.id))
    }
}

#[derive(Debug)]
pub struct Formats<'a> {
    iter: core::slice::Iter<'a, Format>,
}

impl Iterator for Formats<'_> {
    type Item = Format;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }
}
