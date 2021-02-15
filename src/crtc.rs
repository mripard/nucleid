use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    device::Inner,
    object::{Object, Type as ObjectType},
    raw::drm_mode_get_crtc,
    Device, Error, Result,
};

/// A KMS CRTC
///
/// A CRTC is the central part of the display pipeline. It receives the [`Planes`](crate::Plane)
/// content and blends them together. It will then generate the timings through the attached
/// [`Mode`](crate::Mode) and will feed the output to a [`Connector`](crate::Connector).
#[derive(Debug)]
pub struct Crtc {
    dev: Weak<RefCell<Inner>>,
    id: u32,
    idx: usize,
}

impl Crtc {
    pub(crate) fn new(device: &Device, id: u32, idx: usize) -> Result<Self> {
        let _ = drm_mode_get_crtc(device, id)?;

        Ok(Self {
            dev: Rc::downgrade(&device.inner),
            id,
            idx,
        })
    }

    pub(crate) const fn index(&self) -> usize {
        self.idx
    }
}

impl Object for Crtc {
    fn device(&self) -> Result<Device> {
        Ok(self.dev.upgrade().ok_or(Error::Empty)?.into())
    }

    fn object_id(&self) -> u32 {
        self.id
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::Crtc
    }
}
