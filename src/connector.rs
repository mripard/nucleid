use core::fmt;
use std::{
    cell::RefCell,
    convert::TryFrom,
    io,
    rc::{Rc, Weak},
};

use crate::{
    device::Inner,
    encoder::Encoder,
    mode::drm_mode_type as ModeType,
    object::Object,
    raw::{
        drm_connector_status, drm_mode_connector_type, drm_mode_get_connector, drm_mode_object_type,
    },
    Device, Mode,
};

/// A Display Sink Connector
///
/// A connector is the abstraction for any display sinks, including some that might not have a
/// physical connector, such as fixed panels.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Connector {
    dev: Weak<RefCell<Inner>>,
    id: u32,
    type_: drm_mode_connector_type,
    type_id: u32,
    mm_height: usize,
    mm_width: usize,
    encoder_ids: Vec<u32>,
}

#[derive(Debug)]
pub struct Modes(Vec<Mode>);

impl IntoIterator for Modes {
    type Item = Mode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Connector {
    pub(crate) fn new(device: &Device, id: u32) -> io::Result<Self> {
        let mut encoder_ids = Vec::new();
        let connector = drm_mode_get_connector(device, id, None, Some(&mut encoder_ids))?;
        let con_type = drm_mode_connector_type::try_from(connector.connector_type).unwrap();

        Ok(Self {
            dev: Rc::downgrade(&device.inner),
            id,
            type_: con_type,
            type_id: connector.connector_type_id,
            mm_height: connector.mm_height as usize,
            mm_width: connector.mm_width as usize,
            encoder_ids,
        })
    }

    /// Returns an iterator over the [Mode]s supported by the [Connector]
    ///
    /// This list of [Mode]s isn't exhaustive, and additional [Mode]s can be supported depending on
    /// the hardware, driver and display sink.
    ///
    /// # Errors
    ///
    /// Will return [Error] if the [Device] can't be accessed or if the ioctl fails.
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
    /// let modes = connector.modes().unwrap();
    /// ```
    pub fn modes(&self) -> io::Result<Modes> {
        let device: Device = self
            .dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into();

        let mut raw_modes = Vec::new();
        let _ = drm_mode_get_connector(&device, self.id, Some(&mut raw_modes), None)?;

        let mut modes = Vec::with_capacity(raw_modes.len());
        for mode in &raw_modes {
            modes.push(Mode::new(*mode));
        }

        Ok(Modes(modes))
    }

    /// Returns the preferred [Mode] for the [Connector]
    ///
    /// # Errors
    ///
    /// Will return [Error] if the [Device] can't be accessed or if the ioctl fails.
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
    /// let mode = connector.preferred_mode().unwrap();
    /// ```
    pub fn preferred_mode(&self) -> io::Result<Mode> {
        self.modes()?
            .into_iter()
            .find(|mode| mode.has_type(ModeType::Preferred))
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "No Preferred Mode Found",
            ))
    }

    /// Returns the [Connector] current status
    ///
    /// # Errors
    ///
    /// Will return [Error] if the [Device] can't be accessed or if the ioctl fails.
    ///
    /// # Panics
    ///
    /// If the connection status cannot be decoded
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{raw::drm_connector_status, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == drm_connector_status::Connected);
    /// ```
    pub fn status(&self) -> io::Result<drm_connector_status> {
        let device: Device = self
            .dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into();

        let connector = drm_mode_get_connector(&device, self.id, None, None)?;

        Ok(drm_connector_status::try_from(connector.connection).unwrap())
    }

    /// Returns the [Connector] type
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{raw::drm_mode_connector_type, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.connector_type() == drm_mode_connector_type::HDMIA);
    /// ```
    #[must_use]
    pub const fn connector_type(&self) -> drm_mode_connector_type {
        self.type_
    }

    /// Returns the [Connector] type index
    ///
    /// [Connector]s are reported by the kernel by using a global ID, but also by using a
    /// combination of the [drm_mode_connector_type] and the ID of that [Connector] within that
    /// type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{raw::drm_mode_connector_type, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.connector_type() == drm_mode_connector_type::HDMIA)
    ///     .unwrap();
    ///
    /// assert_eq!(connector.connector_type_id(), 0);
    /// ```
    #[must_use]
    pub const fn connector_type_id(&self) -> u32 {
        self.type_id
    }

    pub(crate) fn encoders(self: &Rc<Self>) -> io::Result<Encoders> {
        let device: Device = self
            .dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into();

        let encoders = device
            .encoders()
            .filter(|enc| self.encoder_ids.contains(&enc.id()))
            .collect();

        Ok(Encoders(encoders))
    }
}

impl Object for Connector {
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
        drm_mode_object_type::Connector
    }
}

impl fmt::Display for Connector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.type_, self.type_id))
    }
}

#[derive(Debug)]
pub struct Encoders(Vec<Rc<Encoder>>);

impl IntoIterator for Encoders {
    type Item = Rc<Encoder>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
