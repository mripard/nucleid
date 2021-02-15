use crate::{raw::drm_mode_get_property, Device, Result};

/// A KMS property
#[derive(Debug)]
pub struct Property {
    object_id: u32,
    id: u32,
    name: String,
    value: u64,
}

impl Property {
    pub(crate) fn new(device: &Device, object_id: u32, id: u32, value: u64) -> Result<Self> {
        let property = drm_mode_get_property(device, id)?;
        let name = std::str::from_utf8(&property.name)?
            .trim_end_matches(char::from(0))
            .to_string();

        Ok(Self {
            object_id,
            id,
            name,
            value,
        })
    }

    #[must_use]
    pub(crate) const fn id(&self) -> u32 {
        self.id
    }

    /// Returns the [Property] name
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::Device;
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let plane = device.planes()
    ///     .into_iter()
    ///     .find(|plane| {
    ///         plane
    ///             .properties()
    ///             .unwrap()
    ///             .into_iter()
    ///             .find(|prop| prop.name() == "COLOR_RANGE")
    ///             .is_some()
    ///     })
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the [Property] value
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nucleid::{Device, PlaneType};
    ///
    /// let device = Device::new("/dev/dri/card0").unwrap();
    ///
    /// let plane = device.planes()
    ///     .into_iter()
    ///     .find(|plane| plane.plane_type() == PlaneType::Primary)
    ///     .unwrap();
    ///
    /// let plane_type = plane
    ///     .properties()
    ///     .unwrap()
    ///     .into_iter()
    ///     .find(|prop| prop.name() == "type")
    ///     .unwrap();
    ///
    /// assert_eq!(plane_type.value(), PlaneType::Primary as u64);
    /// ```
    #[must_use]
    pub const fn value(&self) -> u64 {
        self.value
    }
}
