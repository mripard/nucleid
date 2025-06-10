use std::{
    cell::{Ref, RefCell},
    fs::OpenOptions,
    io,
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd},
    path::Path,
    rc::Rc,
};

use crate::{
    encoder::Encoder,
    raw::{drm_mode_get_planes, drm_mode_get_resources, drm_set_client_capability},
    Buffer, BufferType, Connector, Crtc, Output, Plane,
};

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u64)]
enum ClientCapability {
    Stereo3d = 1,
    UniversalPlanes,
    Atomic,
    AspectRatio,
    WritebackConnectors,
}

#[derive(Debug)]
pub struct Inner {
    pub(crate) file: OwnedFd,
    crtcs: Vec<Rc<Crtc>>,
    encoders: Vec<Rc<Encoder>>,
    connectors: Vec<Rc<Connector>>,
    planes: Vec<Rc<Plane>>,
}

#[derive(Debug)]
pub struct Connectors<'a> {
    inner: Ref<'a, Inner>,
    count: usize,
}

impl Iterator for Connectors<'_> {
    type Item = Rc<Connector>;

    fn next(&mut self) -> Option<Self::Item> {
        let child = self.inner.connectors.get(self.count);
        self.count += 1;

        child.map(Rc::clone)
    }
}

#[derive(Debug)]
pub struct Crtcs<'a> {
    inner: Ref<'a, Inner>,
    count: usize,
}

impl Iterator for Crtcs<'_> {
    type Item = Rc<Crtc>;

    fn next(&mut self) -> Option<Self::Item> {
        let child = self.inner.crtcs.get(self.count);
        self.count += 1;

        child.map(Rc::clone)
    }
}

#[derive(Debug)]
pub struct Encoders<'a> {
    inner: Ref<'a, Inner>,
    count: usize,
}

impl Iterator for Encoders<'_> {
    type Item = Rc<Encoder>;

    fn next(&mut self) -> Option<Self::Item> {
        let child = self.inner.encoders.get(self.count);
        self.count += 1;

        child.map(Rc::clone)
    }
}

#[derive(Debug)]
pub struct Planes<'a> {
    inner: Ref<'a, Inner>,
    count: usize,
}

impl Iterator for Planes<'_> {
    type Item = Rc<Plane>;

    fn next(&mut self) -> Option<Self::Item> {
        let child = self.inner.planes.get(self.count);
        self.count += 1;

        child.map(Rc::clone)
    }
}

/// The DRM Device
///
/// A Device abstracts a collection of hardware components that glued and used together will provide
/// the display capabilities and a number of [Plane]s, [Crtc]s and [Connector]s
#[derive(Debug)]
pub struct Device {
    pub(crate) inner: Rc<RefCell<Inner>>,
}

impl Device {
    /// Creates a new [Device] from a path
    ///
    /// # Errors
    ///
    /// If `path` doesn't exist, the user doesn't have permission to access it or if the ioctl
    /// fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::Device;
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    /// ```
    pub fn new<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = OpenOptions::new().read(true).write(true).open(path)?;

        drm_set_client_capability(&file, ClientCapability::Atomic as u64)?;
        drm_set_client_capability(&file, ClientCapability::UniversalPlanes as u64)?;

        let mut crtc_ids = Vec::new();
        let mut encoder_ids = Vec::new();
        let mut connector_ids = Vec::new();
        let _res = drm_mode_get_resources(
            &file,
            Some(&mut crtc_ids),
            Some(&mut encoder_ids),
            Some(&mut connector_ids),
        )?;

        let device = Self {
            inner: Rc::new(RefCell::new(Inner {
                file: file.into(),
                crtcs: Vec::new(),
                encoders: Vec::new(),
                connectors: Vec::new(),
                planes: Vec::new(),
            })),
        };

        for (idx, id) in crtc_ids.into_iter().enumerate() {
            let crtc = Rc::new(Crtc::new(&device, id, idx)?);

            device.inner.borrow_mut().crtcs.push(crtc);
        }

        for id in encoder_ids {
            let encoder = Rc::new(Encoder::new(&device, id)?);

            device.inner.borrow_mut().encoders.push(encoder);
        }

        for id in connector_ids {
            let connector = Rc::new(Connector::new(&device, id)?);

            device.inner.borrow_mut().connectors.push(connector);
        }

        let plane_ids = drm_mode_get_planes(&device)?;
        for id in plane_ids {
            let plane = Rc::new(Plane::new(&device, id)?);

            device.inner.borrow_mut().planes.push(plane);
        }

        Ok(device)
    }

    /// Returns an Iterator over the [Connector]s
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::Device;
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let connectors: Vec<_> = device.connectors()
    ///     .collect();
    /// ```
    #[must_use]
    pub fn connectors(&self) -> Connectors<'_> {
        let inner = self.inner.borrow();

        Connectors { inner, count: 0 }
    }

    /// Returns an Iterator over the [Crtc]s
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::Device;
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let crtcs: Vec<_> = device.crtcs()
    ///     .collect();
    /// ```
    #[must_use]
    pub fn crtcs(&self) -> Crtcs<'_> {
        let inner = self.inner.borrow();

        Crtcs { inner, count: 0 }
    }

    pub(crate) fn encoders(&self) -> Encoders<'_> {
        let inner = self.inner.borrow();

        Encoders { inner, count: 0 }
    }

    /// Returns an Iterator over the [Plane]s
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::Device;
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let planes: Vec<_> = device.planes()
    ///     .collect();
    /// ```
    #[must_use]
    pub fn planes(&self) -> Planes<'_> {
        let inner = self.inner.borrow();

        Planes { inner, count: 0 }
    }

    /// Allocates a DRM [Buffer]
    ///
    /// # Errors
    ///
    /// If the buffer allocation fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::BufferType;
    /// use nucleid::Device;
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let buffer = device.allocate_buffer(BufferType::Dumb, 640, 480, 32)
    ///     .unwrap();
    /// ```
    pub fn allocate_buffer(
        &self,
        buftype: BufferType,
        width: u32,
        height: u32,
        bpp: u32,
    ) -> io::Result<Buffer> {
        let raw = match buftype {
            BufferType::Dumb => Buffer::new(self, width, height, bpp)?,
        };

        Ok(raw)
    }

    /// Builds an [Output] from a [Connector]
    ///
    /// Finds a suitable [Crtc] for a given [Connector] and creates an [Output] from
    /// that.
    ///
    /// # Errors
    ///
    /// If the [Device] can't be accessed, if the ioctl fails, or if it could not find a suitable
    /// [Crtc] for the [Connector]
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
    /// ```
    pub fn output_from_connector(&self, connector: &Rc<Connector>) -> io::Result<Output> {
        let encoder = connector
            .encoders()?
            .into_iter()
            .next()
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Couldn't find an encoder for that connector.",
            ))?;

        let crtc = encoder.crtcs()?.into_iter().next().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "Couldn't find a CRTC for that connector.",
        ))?;

        Ok(Output::new(self, &crtc, &encoder, connector))
    }
}

impl AsFd for Device {
    fn as_fd(&self) -> BorrowedFd<'_> {
        // SAFETY: We know that we will have the fd opened for at least as long as Device.
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

impl std::os::unix::io::AsRawFd for Device {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.inner.borrow().file.as_raw_fd()
    }
}

impl From<Rc<RefCell<Inner>>> for Device {
    fn from(rc: Rc<RefCell<Inner>>) -> Self {
        Self { inner: rc }
    }
}
