use std::{
    cell::RefCell,
    convert::TryFrom,
    rc::{Rc, Weak},
};

use num_enum::TryFromPrimitive;

use crate::error::Result;
use crate::{device::Inner, raw::drm_mode_get_encoder, Crtc, Device, Error};

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
#[allow(clippy::upper_case_acronyms)]
pub enum Type {
    None,
    DAC,
    TMDS,
    LVDS,
    TVDAC,
    Virtual,
    DSI,
    DPMST,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Encoder {
    dev: Weak<RefCell<Inner>>,
    id: u32,
    type_: Type,
    possible_crtcs: u32,
    possible_clones: u32,
}

impl Encoder {
    pub(crate) fn new(device: &Device, id: u32) -> Result<Self> {
        let encoder = drm_mode_get_encoder(device, id)?;
        let encoder_type = Type::try_from(encoder.encoder_type).unwrap();

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

    pub fn crtcs(self: &Rc<Self>) -> Result<Crtcs> {
        let device: Device = self.dev.upgrade().ok_or(Error::Empty)?.into();

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

        Ok(Crtcs(crtcs))
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
