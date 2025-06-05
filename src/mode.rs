use std::ffi::CStr;

use bytemuck::cast_slice;

use crate::raw::{bindgen, drm_mode_modeinfo};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum drm_mode_type {
    Builtin,
    ClockC,
    CrtcC,
    Preferred,
    Default,
    UserDef,
    Driver,
}

/// Display Mode
///
/// Contains the set of timings needed for a given display output
#[derive(Debug)]
#[allow(dead_code)]
pub struct Mode {
    name: String,
    inner: drm_mode_modeinfo,
}

impl Mode {
    pub(crate) fn new(info: drm_mode_modeinfo) -> Self {
        let name = CStr::from_bytes_until_nul(cast_slice(&info.name))
            .expect("The kernel guarantees the string is null-terminated.")
            .to_str()
            .expect("The kernel guarantees this is an ASCII.")
            .to_owned();

        Self { name, inner: info }
    }

    pub(crate) const fn has_type(&self, arg: drm_mode_type) -> bool {
        let mode_type = self.inner.type_;

        let mask = match arg {
            drm_mode_type::Builtin => bindgen::DRM_MODE_TYPE_BUILTIN,
            drm_mode_type::ClockC => bindgen::DRM_MODE_TYPE_CLOCK_C,
            drm_mode_type::CrtcC => bindgen::DRM_MODE_TYPE_CRTC_C,
            drm_mode_type::Preferred => bindgen::DRM_MODE_TYPE_PREFERRED,
            drm_mode_type::Default => bindgen::DRM_MODE_TYPE_DEFAULT,
            drm_mode_type::UserDef => bindgen::DRM_MODE_TYPE_USERDEF,
            drm_mode_type::Driver => bindgen::DRM_MODE_TYPE_DRIVER,
        };

        (mode_type & mask) == mask
    }

    pub(crate) const fn inner(&self) -> &drm_mode_modeinfo {
        &self.inner
    }

    /// Returns the active vertical size in pixels
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let mode = connector.modes().unwrap()
    ///     .into_iter()
    ///     .find(|mode| mode.height() == 1920)
    ///     .unwrap();
    /// ```
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.inner.vdisplay
    }

    /// Returns the vertical refresh rate, in Hertz
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let mode = connector.modes().unwrap()
    ///     .into_iter()
    ///     .find(|mode| mode.refresh() == 60)
    ///     .unwrap();
    /// ```
    #[must_use]
    pub const fn refresh(&self) -> u32 {
        self.inner.vrefresh
    }

    /// Returns the active horizontal size in pixels
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let mode = connector.modes().unwrap()
    ///     .into_iter()
    ///     .find(|mode| mode.width() == 1080)
    ///     .unwrap();
    /// ```
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.inner.hdisplay
    }
}
