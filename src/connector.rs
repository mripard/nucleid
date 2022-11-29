use std::{
    cell::RefCell,
    convert::TryFrom,
    rc::{Rc, Weak},
};

use num_enum::TryFromPrimitive;

use crate::{
    device::Inner,
    encoder::Encoder,
    mode::Type as ModeType,
    object::{Object, Type as ObjectType},
    raw::drm_mode_get_connector,
    Device, Error, Mode, Result,
};

/// [Connector] Status
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum Status {
    /// This [Connector] is connected to a sink and can be enabled
    Connected = 1,

    /// This [Connector] hasn't detected a sink. Whether the [Connector] can be enabled or not is
    /// driver-dependant.
    Disconnected,

    /// This [Connector] status couldn't reliably be determined. The [Connector] can be enabled
    /// with a fallback mode.
    Unknown,
}

/// The [Connector] Type
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum Type {
    /// The [Connector] type couldn't be determined
    Unknown,

    /// A VGA DE-15 [Connector]
    VGA,

    /// A DVI-I [Connector]
    DVII,

    /// A DVI-D [Connector]
    DVID,

    /// A DVI-A [Connector]
    DVIA,

    /// An RCA [Connector] carrying a CVBS signal
    Composite,

    /// An S-Video [Connector]
    SVIDEO,

    /// An LVDS [Connector]
    LVDS,

    /// A Component [Connector]
    Component,

    /// A mini-Din-9 [Connector]
    MiniDin9,

    /// A DisplayPort [Connector]
    DisplayPort,

    /// An HDMI-A [Connector]
    HDMIA,

    /// An HDMI-B [Connector]
    HDMIB,

    /// A TV [Connector]
    TV,

    /// An embedded DisplayPort [Connector]
    EDP,

    /// A Virtual [Connector]
    Virtual,

    /// A MIPI-DSI [Connector]
    DSI,

    /// A MIPI-DPI [Connector]
    DPI,

    /// A Writeback [Connector]
    Writeback,

    /// An SPI-based Display [Connector]
    SPI,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Component => write!(f, "Component"),
            Self::Composite => write!(f, "Composite"),
            Self::DPI => write!(f, "DPI"),
            Self::DSI => write!(f, "DSI"),
            Self::DVIA => write!(f, "DVI-A"),
            Self::DVID => write!(f, "DVI-D"),
            Self::DVII => write!(f, "DVI-I"),
            Self::DisplayPort => write!(f, "DisplayPort"),
            Self::EDP => write!(f, "eDP"),
            Self::HDMIA => write!(f, "HDMI-A"),
            Self::HDMIB => write!(f, "HDMI-B"),
            Self::LVDS => write!(f, "LVDS"),
            Self::MiniDin9 => write!(f, "MiniDin9"),
            Self::SPI => write!(f, "SPI"),
            Self::SVIDEO => write!(f, "S-VIDEO"),
            Self::TV => write!(f, "TV"),
            Self::Unknown => write!(f, "Unknown"),
            Self::VGA => write!(f, "VGA"),
            Self::Virtual => write!(f, "Virtual"),
            Self::Writeback => write!(f, "Writeback"),
        }
    }
}

/// A Display Sink Connector
///
/// A connector is the abstraction for any display sinks, including some that might not have a
/// physical connector, such as fixed panels.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Connector {
    dev: Weak<RefCell<Inner>>,
    id: u32,
    type_: Type,
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
    pub(crate) fn new(device: &Device, id: u32) -> Result<Self> {
        let mut encoder_ids = Vec::new();
        let connector = drm_mode_get_connector(device, id, None, Some(&mut encoder_ids))?;
        let con_type = Type::try_from(connector.connector_type).unwrap();

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
    pub fn modes(&self) -> Result<Modes> {
        let device: Device = self.dev.upgrade().ok_or(Error::Empty)?.into();

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
    pub fn preferred_mode(&self) -> Result<Mode> {
        self.modes()?
            .into_iter()
            .find(|mode| mode.has_type(ModeType::Preferred))
            .ok_or(Error::Empty)
    }

    /// Returns the [Connector] current status
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
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected);
    /// ```
    pub fn status(&self) -> Result<Status> {
        let device: Device = self.dev.upgrade().ok_or(Error::Empty)?.into();

        let connector = drm_mode_get_connector(&device, self.id, None, None)?;

        Ok(Status::try_from(connector.connection).unwrap())
    }

    /// Returns the [Connector] type
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorType, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.connector_type() == ConnectorType::HDMIA);
    /// ```
    #[must_use]
    pub const fn connector_type(&self) -> Type {
        self.type_
    }

    /// Returns the [Connector] type index
    ///
    /// [Connector]s are reported by the kernel by using a global ID, but also by using a
    /// combination of the [Type] and the ID of that [Connector] within that type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorType, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.connector_type() == ConnectorType::HDMIA)
    ///     .unwrap();
    ///
    /// assert_eq!(connector.connector_type_id(), 0);
    /// ```
    #[must_use]
    pub const fn connector_type_id(&self) -> u32 {
        self.type_id
    }

    pub(crate) fn encoders(self: &Rc<Self>) -> Result<Encoders> {
        let device: Device = self.dev.upgrade().ok_or(Error::Empty)?.into();

        let encoders = device
            .encoders()
            .filter(|enc| self.encoder_ids.contains(&enc.id()))
            .collect();

        Ok(Encoders(encoders))
    }
}

impl Object for Connector {
    fn device(&self) -> Result<Device> {
        Ok(self.dev.upgrade().ok_or(Error::Empty)?.into())
    }

    fn object_id(&self) -> u32 {
        self.id
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::Connector
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
