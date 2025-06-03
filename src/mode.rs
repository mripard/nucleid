use crate::raw::drm_mode_modeinfo;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Type {
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
        let name = std::str::from_utf8(&info.name)
            .unwrap()
            .trim_end_matches(char::from(0))
            .to_string();

        Self { name, inner: info }
    }

    pub(crate) const fn has_type(&self, arg: Type) -> bool {
        let mode_type = self.inner.type_;

        let mask = match arg {
            Type::Builtin => 1,
            Type::ClockC => (1 << 1) | 1,
            Type::CrtcC => (1 << 2) | 1,
            Type::Preferred => 1 << 3,
            Type::Default => 1 << 4,
            Type::UserDef => 1 << 5,
            Type::Driver => 1 << 6,
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
        self.inner.vrefresh as u32
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
