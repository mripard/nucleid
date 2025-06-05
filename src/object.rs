use std::io;

use crate::{
    raw::{drm_mode_get_properties, drm_mode_object_type},
    Device, Property,
};

/// A KMS Object
pub trait Object {
    /// Returns the [Device] the [Object] belongs to
    fn device(&self) -> Device;

    /// Returns the [Object] ID
    fn object_id(&self) -> u32;

    /// Returns the [Object] type
    fn object_type(&self) -> drm_mode_object_type;

    /// Returns a list of [Property] attached to that object.
    ///
    /// # Errors
    ///
    /// If there's an I/O Error while accessing the [Device] file descriptor
    fn properties(&self) -> io::Result<Vec<Property>> {
        let dev = self.device();
        let object_id = self.object_id();

        let properties = drm_mode_get_properties(&dev, self.object_type() as u32, object_id)?;

        let mut ret = Vec::new();
        for (prop_id, prop_value) in properties {
            let property = Property::new(&dev, object_id, prop_id, prop_value)?;

            ret.push(property);
        }

        Ok(ret)
    }

    /// Looks up a [Property] by name on a given [Object]
    ///
    /// # Errors
    ///
    /// If there's an I/O Error while accessing the [Device] file descriptor
    fn property(&self, name: &str) -> io::Result<Option<Property>> {
        Ok(self.properties()?.into_iter().find(|p| p.name() == name))
    }

    /// Looks up a [Property] ID by name on a given [Object]
    ///
    /// # Errors
    ///
    /// If there's an I/O Error while accessing the [Device] file descriptor
    fn property_id(&self, name: &str) -> io::Result<Option<u32>> {
        Ok(self.property(name)?.map(|p| p.id()))
    }
}
