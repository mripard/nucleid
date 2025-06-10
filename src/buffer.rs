use std::{
    cell::RefCell,
    convert::TryInto,
    io,
    rc::{Rc, Weak},
};

use memmap::{MmapMut, MmapOptions};

use crate::{
    device::Inner,
    raw::{
        drm_mode_add_framebuffer, drm_mode_create_dumb_buffer, drm_mode_destroy_dumb_buffer,
        drm_mode_map_dumb_buffer, drm_mode_remove_framebuffer,
    },
    Device, Format,
};

/// A DRM Buffer Type
///
/// This Buffer Type contains either the generic DRM Buffer Types or the vendor specific ones.
#[derive(Clone, Copy, Debug)]
pub enum Type {
    /// A DRM Dumb Buffer, only accessible by the scanout
    Dumb,
}

/// A DRM Buffer
///
/// A buffer to be used with the rest of the nucleid API. This needs to be turned into a
/// [Framebuffer] before being sent to the Device
pub struct Buffer {
    dev: Weak<RefCell<Inner>>,
    width: u32,
    height: u32,
    pitch: u32,
    size: u64,
    handle: u32,
    mapping: MmapMut,
}

impl Buffer {
    pub(crate) fn new(device: &Device, width: u32, height: u32, bpp: u32) -> io::Result<Self> {
        let dumb = drm_mode_create_dumb_buffer(device, width, height, bpp)?;
        let map = drm_mode_map_dumb_buffer(device, dumb.handle)?;

        // NOTE: dumb.size is a u64, and usize will be a u32 on 32-bits platforms. However, a size
        // larger than 32-bits on those platforms wouldn't make sense, so let's panic if we
        // encounter it.
        let size = dumb.size.try_into().unwrap();

        let map = unsafe {
            MmapOptions::new()
                .len(size)
                .offset(map.offset)
                .map_mut(&device.inner.borrow().file)
        }?;

        Ok(Self {
            dev: Rc::downgrade(&device.inner),

            width: dumb.width,
            height: dumb.height,
            pitch: dumb.pitch,
            size: dumb.size,

            handle: dumb.handle,
            mapping: map,
        })
    }

    /// Extracts a mutable slice of the entire [Buffer] if it is mapped
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{BufferType, Device};
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let mut buffer = device.allocate_buffer(BufferType::Dumb, 640, 480, 32)
    ///     .unwrap();
    ///
    /// let data = buffer.data();
    /// data.copy_from_slice(&[0xff, 0xff, 0xff]);
    /// ```
    #[must_use]
    pub fn data(&mut self) -> &mut [u8] {
        &mut self.mapping
    }

    /// Returns the height, in lines
    ///
    /// This height can be larger than the one provided during the allocation of the [Buffer].
    /// Indeed, the kernel is free to increase it to accomodate for constraints the hardware
    /// might have.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{BufferType, Device};
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let buffer = device.allocate_buffer(BufferType::Dumb, 640, 480, 32)
    ///     .unwrap();
    ///
    /// assert!(buffer.height() >= 480)
    /// ```
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the width, in pixels
    ///
    /// This width can be larger than the one provided during the allocation of the [Buffer].
    /// Indeed, the kernel is free to increase it to accomodate for constraints the hardware
    /// might have.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{BufferType, Device};
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let buffer = device.allocate_buffer(BufferType::Dumb, 640, 480, 32)
    ///     .unwrap();
    ///
    /// assert!(buffer.width() >= 640)
    /// ```
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the pitch, in bytes
    ///
    /// This pitch can be larger than the product of the width and bytes per pixel provided
    /// during the allocation of the [Buffer]. Indeed, the kernel is free to increase it to
    /// accomodate for constraints the hardware might have.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{BufferType, Device};
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let buffer = device.allocate_buffer(BufferType::Dumb, 640, 480, 32)
    ///     .unwrap();
    ///
    /// assert!(buffer.pitch() >= (640 * 32))
    /// ```
    #[must_use]
    pub const fn pitch(&self) -> u32 {
        self.pitch
    }

    /// Returns the size, in bytes
    ///
    /// This pitch can be larger than the product of the width, height and bytes per pixel provided
    /// during the allocation of the [Buffer]. Indeed, the kernel is free to increase it to
    /// accomodate for constraints the hardware might have.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{BufferType, Device};
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let buffer = device.allocate_buffer(BufferType::Dumb, 640, 480, 32)
    ///     .unwrap();
    ///
    /// assert!(buffer.size() >= (640 * 480 * 32))
    /// ```
    #[must_use]
    pub const fn size(&self) -> u64 {
        self.size
    }

    /// Request the creation of a [Framebuffer]
    ///
    /// A DRM buffer needs to be added as a [Framebuffer] in order to attach them to a
    /// [Plane](crate::Plane), and eventually to send them to the scanout.
    ///
    /// # Errors
    ///
    /// If the [Device] can't be accessed or if the ioctl fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{BufferType, Device, Format};
    ///
    /// let device = Device::new("/dev/dri/card0")
    ///     .unwrap();
    ///
    /// let fb = device.allocate_buffer(BufferType::Dumb, 640, 480, 32)
    ///     .unwrap()
    ///     .into_framebuffer(Format::XRGB8888)
    ///     .unwrap();
    /// ```
    pub fn into_framebuffer(self, fmt: Format) -> io::Result<Framebuffer> {
        let device: Device = self
            .dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into();

        let id = drm_mode_add_framebuffer(
            &device,
            self.handle,
            self.width,
            self.pitch,
            self.height,
            fmt as u32,
        )?;

        Ok(Framebuffer {
            dev: Rc::downgrade(&device.inner),
            buffer: self,
            id,
        })
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        let device: Device = self
            .dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into();

        let _res = drm_mode_destroy_dumb_buffer(&device, self.handle);
    }
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Buffer")
            .field("handle", &self.handle)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("pitch", &self.pitch)
            .field("size", &self.size)
            .finish_non_exhaustive()
    }
}

/// A DRM Frame Buffer
///
/// A Frame Buffer is an abstraction to provide the source of the pixels to the [CRTC](crate::Crtc).
/// They are then attached to a [`Plane`](crate::Plane) through a
/// [`PlaneUpdate`](crate::PlaneUpdate).
#[derive(Debug)]
pub struct Framebuffer {
    dev: Weak<RefCell<Inner>>,
    buffer: Buffer,
    id: u32,
}

impl Framebuffer {
    pub(crate) const fn id(&self) -> u32 {
        self.id
    }
}

impl std::ops::Deref for Framebuffer {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl std::ops::DerefMut for Framebuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        let device: Device = self
            .dev
            .upgrade()
            .expect("Couldn't upgrade our weak reference")
            .into();

        let _res = drm_mode_remove_framebuffer(&device, self.id);
    }
}
