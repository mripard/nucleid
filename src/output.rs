use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use fixed::types::U16F16;

use crate::{
    buffer::Framebuffer, device::Inner, encoder::Encoder, object::Object,
    raw::drm_mode_atomic_commit, raw::drm_mode_create_property_blob, Connector, Crtc, Device,
    Error, Mode, Plane, Result,
};

/// Display Pipeline Output Abstraction
#[derive(Debug)]
#[allow(dead_code)]
pub struct Output {
    dev: Weak<RefCell<Inner>>,
    connector: Rc<Connector>,
    crtc: Rc<Crtc>,
    encoder: Rc<Encoder>,
}

impl Output {
    pub(crate) fn new(
        device: &Device,
        crtc: &Rc<Crtc>,
        encoder: &Rc<Encoder>,
        connector: &Rc<Connector>,
    ) -> Self {
        Self {
            dev: Rc::downgrade(&device.inner),
            connector: Rc::clone(connector),
            crtc: Rc::clone(crtc),
            encoder: Rc::clone(encoder),
        }
    }

    /// Returns the backing [Crtc]
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
    /// let output = device.output_from_connector(&connector).unwrap();
    /// let crtc = output.crtc();
    /// ```
    #[must_use]
    pub fn crtc(self) -> Rc<Crtc> {
        Rc::clone(&self.crtc)
    }

    /// Returns an iterator over the [Plane]s available
    ///
    /// # Panics
    ///
    /// If the back-pointer to the DRM device isn't valid anymore.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device.output_from_connector(&connector).unwrap();
    /// let plane = output
    ///     .planes()
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
    pub fn planes(&self) -> Planes {
        let device: Device = self.dev.upgrade().ok_or(Error::Empty).unwrap().into();
        let crtc_idx = self.crtc.index();

        let planes = device
            .planes()
            .filter(|plane| (((1 << crtc_idx) & plane.possible_crtcs()) != 0))
            .collect();

        Planes(planes)
    }

    /// Starts an [Update] of the current [Output]
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
    /// let output = device.output_from_connector(&connector).unwrap();
    /// let output = output.start_update().commit().unwrap();
    /// ```
    #[must_use]
    pub const fn start_update(self) -> Update {
        Update {
            mode: None,
            output: self,
            connector: None,
            planes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Planes(Vec<Rc<Plane>>);

impl IntoIterator for Planes {
    type Item = Rc<Plane>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// [Output] state modification abstraction
#[derive(Debug)]
pub struct Update {
    mode: Option<Mode>,
    output: Output,
    connector: Option<ConnectorUpdate>,
    planes: Vec<PlaneUpdate>,
}

impl Update {
    /// Adds a [`ConnectorUpdate`] to the pending [Update]
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, ConnectorUpdate, Device};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_connector(ConnectorUpdate::new(&connector))
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn add_connector(mut self, connector: ConnectorUpdate) -> Self {
        self.connector = Some(connector);
        self
    }

    /// Adds a [`PlaneUpdate`] to the pending [Update]
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(PlaneUpdate::new(&plane))
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn add_plane(mut self, plane: PlaneUpdate) -> Self {
        self.planes.push(plane);
        self
    }

    /// Commits the pending [Update]
    ///
    /// # Errors
    ///
    /// Will return [Error] if the [Device] can't be accessed, if the ioctl fails, or if the
    /// [Update] is rejected by the hardware.
    ///
    /// # Panics
    ///
    /// If the back-pointer to the DRM device isn't valid anymore.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, ConnectorUpdate, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_connector(ConnectorUpdate::new(&connector))
    ///     .add_plane(PlaneUpdate::new(&plane))
    ///     .commit()
    ///     .unwrap();
    /// ```
    pub fn commit(self) -> Result<Output> {
        let device: Device = self.output.dev.upgrade().ok_or(Error::Empty)?.into();
        let mut properties = Vec::new();
        let crtc_object_id = self.output.crtc.object_id();

        for plane in self.planes {
            let crtc_prop_id = plane.plane.property_id("CRTC_ID").unwrap();
            properties.push((
                plane.plane.object_id(),
                crtc_prop_id,
                u64::from(crtc_object_id),
            ));

            for (prop_name, prop_value) in plane.properties {
                let prop_id = plane.plane.property_id(&prop_name).ok_or(Error::Empty)?;

                properties.push((plane.plane.object_id(), prop_id, prop_value));
            }
        }

        let active_prop_id = self.output.crtc.property_id("ACTIVE").unwrap();
        properties.push((crtc_object_id, active_prop_id, 1));

        if let Some(mode) = self.mode {
            let mode_id = u64::from(drm_mode_create_property_blob(&device, mode.inner())?);
            let mode_prop_id = self.output.crtc.property_id("MODE_ID").unwrap();
            properties.push((crtc_object_id, mode_prop_id, mode_id));
        }

        if let Some(connector) = self.connector {
            let crtc_prop_id = connector.connector.property_id("CRTC_ID").unwrap();
            properties.push((
                connector.connector.object_id(),
                crtc_prop_id,
                u64::from(crtc_object_id),
            ));

            for (prop_name, prop_value) in connector.properties {
                let prop_id = connector
                    .connector
                    .property_id(&prop_name)
                    .ok_or(Error::Empty)?;

                properties.push((connector.connector.object_id(), prop_id, prop_value));
            }
        }

        let mut count_props = 0;
        let mut objs_ptr: Vec<u32> = Vec::new();
        let mut count_props_ptr: Vec<u32> = Vec::new();
        let mut props_ptr: Vec<u32> = Vec::new();
        let mut prop_values_ptr: Vec<u64> = Vec::new();

        properties.sort_unstable();
        properties.dedup();

        let first_obj = properties[0].0;
        let mut last_obj = first_obj;

        objs_ptr.push(first_obj);
        for property in properties {
            let oid = property.0;

            if oid != last_obj {
                objs_ptr.push(oid);
                count_props_ptr.push(count_props);

                last_obj = oid;
                count_props = 0;
            }

            count_props += 1;
            props_ptr.push(property.1);
            prop_values_ptr.push(property.2);
        }
        count_props_ptr.push(count_props);

        drm_mode_atomic_commit(
            &device,
            &objs_ptr,
            &count_props_ptr,
            &props_ptr,
            &prop_values_ptr,
        )?;

        Ok(self.output)
    }

    /// Changes the [Mode] of the pending [Update]
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, ConnectorUpdate, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let mode = connector
    ///     .preferred_mode()
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .set_mode(mode)
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn set_mode(mut self, mode: Mode) -> Self {
        self.mode = Some(mode);
        self
    }
}

/// Used to update the state of any KMS Object
pub trait ObjectUpdate {
    /// Adds a [Property](crate::Property) to the new state update  
    #[must_use]
    fn set_property(self, property: &str, val: u64) -> Self;
}

/// [Connector] state update abstraction
#[derive(Debug)]
pub struct ConnectorUpdate {
    connector: Rc<Connector>,
    properties: HashMap<String, u64>,
}

impl ConnectorUpdate {
    /// Creates a new [Connector] state
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, ConnectorUpdate, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_connector(ConnectorUpdate::new(&connector))
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn new(connector: &Rc<Connector>) -> Self {
        Self {
            connector: Rc::clone(connector),
            properties: HashMap::new(),
        }
    }
}

impl ObjectUpdate for ConnectorUpdate {
    fn set_property(mut self, property: &str, val: u64) -> Self {
        self.properties.insert(property.to_string(), val);
        self
    }
}

/// [Plane] state update abstraction
#[derive(Debug)]
pub struct PlaneUpdate {
    plane: Rc<Plane>,
    properties: HashMap<String, u64>,
}

impl PlaneUpdate {
    /// Creates a new [Plane] state
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(PlaneUpdate::new(&plane))
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn new(plane: &Rc<Plane>) -> Self {
        Self {
            plane: Rc::clone(plane),
            properties: HashMap::new(),
        }
    }

    /// Attaches a new [Framebuffer] to the pending [Plane] update
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{BufferType, ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let buffer = device
    ///     .allocate_buffer(BufferType::Dumb, 1920, 1080, 32)
    ///     .unwrap()
    ///     .into_framebuffer(Format::XRGB8888)
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(
    ///         PlaneUpdate::new(&plane)
    ///             .set_framebuffer(&buffer)
    ///     )
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn set_framebuffer(self, fb: &Framebuffer) -> Self {
        let fb_id = fb.id();
        self.set_property("FB_ID", u64::from(fb_id))
    }

    /// Sets the display coordinates in the pending [Plane] update
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(
    ///         PlaneUpdate::new(&plane)
    ///             .set_display_coordinates(640, 0)
    ///     )
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn set_display_coordinates(self, x: usize, y: usize) -> Self {
        self.set_property("CRTC_X", x as u64)
            .set_property("CRTC_Y", y as u64)
    }

    /// Sets the display size in the pending [Plane] update
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(
    ///         PlaneUpdate::new(&plane)
    ///             .set_display_size(1920, 1080)
    ///     )
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn set_display_size(self, width: usize, height: usize) -> Self {
        self.set_property("CRTC_H", height as u64)
            .set_property("CRTC_W", width as u64)
    }

    /// Sets the source coordinates in the pending [Plane] update
    ///
    /// The coordinates are [f32] to support sub-pixel positioning.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(
    ///         PlaneUpdate::new(&plane)
    ///             .set_source_coordinates(860.0, 0.0)
    ///     )
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn set_source_coordinates(self, x: f32, y: f32) -> Self {
        let fixed_x = U16F16::from_num(x);
        let fixed_y = U16F16::from_num(y);

        self.set_property("SRC_X", u64::from(fixed_x.to_bits()))
            .set_property("SRC_Y", u64::from(fixed_y.to_bits()))
    }

    /// Sets the source size in the pending [Plane] update
    ///
    /// The dimensions are [f32] to support sub-pixel positioning.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(
    ///         PlaneUpdate::new(&plane)
    ///             .set_source_size(860.0, 0.0)
    ///     )
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn set_source_size(self, width: f32, height: f32) -> Self {
        let fixed_width = U16F16::from_num(width);
        let fixed_height = U16F16::from_num(height);

        self.set_property("SRC_H", u64::from(fixed_height.to_bits()))
            .set_property("SRC_W", u64::from(fixed_width.to_bits()))
    }

    /// Attaches an arbitrary property to the pending [Plane] update
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{ConnectorStatus, Device, Format, PlaneUpdate};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let connector = device.connectors()
    ///     .into_iter()
    ///     .find(|con| con.status().unwrap() == ConnectorStatus::Connected)
    ///     .unwrap();
    ///
    /// let output = device
    ///     .output_from_connector(&connector)
    ///     .unwrap();
    ///
    /// let plane = output
    ///     .planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .formats()
    ///             .find(|fmt| *fmt == Format::XRGB8888)
    ///             .is_some()
    ///     })
    ///     .unwrap();
    ///
    /// let output = output
    ///     .start_update()
    ///     .add_plane(
    ///         PlaneUpdate::new(&plane)
    ///             .set_property("test property", 42)
    ///     )
    ///     .commit()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn set_property(mut self, property: &str, val: u64) -> Self {
        self.properties.insert(property.to_string(), val);
        self
    }
}
