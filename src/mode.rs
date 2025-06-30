use core::{ffi::CStr, fmt};

use bytemuck::cast_slice;

use crate::raw::{drm_mode_modeinfo, drm_mode_type};

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

    pub(crate) fn has_type(&self, arg: drm_mode_type) -> bool {
        let mode_type = self.inner.type_;
        let mask = u32::from(arg);

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

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}: {} {} {} {} {} {} {} {} {} {} {:x} {:x}",
            self.name,
            self.inner.vrefresh,
            self.inner.clock,
            self.inner.hdisplay,
            self.inner.hsync_start,
            self.inner.hsync_end,
            self.inner.htotal,
            self.inner.vdisplay,
            self.inner.vsync_start,
            self.inner.vsync_end,
            self.inner.vtotal,
            self.inner.type_,
            self.inner.flags
        ))
    }
}
