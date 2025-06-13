use std::{
    cell::RefCell,
    convert::TryFrom,
    io,
    rc::{Rc, Weak},
};

use crate::{
    device::Inner,
    raw::{drm_mode_encoder_type, drm_mode_get_encoder},
    Crtc, Device,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Encoder {
    dev: Weak<RefCell<Inner>>,
    id: u32,
    type_: drm_mode_encoder_type,
    possible_crtcs: u32,
    possible_clones: u32,
}

impl Encoder {
    pub(crate) fn new(device: &Device, id: u32) -> io::Result<Self> {
        let encoder = drm_mode_get_encoder(device, id)?;
        let encoder_type = drm_mode_encoder_type::try_from(encoder.encoder_type).unwrap();

        Ok(Self {
            dev: Rc::downgrade(&device.inner),
            id,
            type_: encoder_type,
            possible_crtcs: encoder.possible_crtcs,
            possible_clones: encoder.possible_clones,
        })
    }

    pub const fn id(&self) -> u32 {
        self.id
    }

    pub fn crtcs(self: &Rc<Self>) -> Crtcs {
        let device: Device = self
            .dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into();

        let crtcs = device
            .crtcs()
            .enumerate()
            .filter_map(|(idx, crtc)| {
                if ((1 << idx) & self.possible_crtcs) == 0 {
                    None
                } else {
                    Some(crtc)
                }
            })
            .collect();

        Crtcs(crtcs)
    }
}

#[derive(Debug)]
pub struct Crtcs(Vec<Rc<Crtc>>);

impl IntoIterator for Crtcs {
    type Item = Rc<Crtc>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
