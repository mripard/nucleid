use std::io;

use crate::{
    raw::{drm_mode_get_properties, drm_mode_object_type},
    Device, Property,
};

pub trait Object {
    fn device(&self) -> Device;
    fn object_id(&self) -> u32;
    fn object_type(&self) -> drm_mode_object_type;

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

    fn property(&self, name: &str) -> io::Result<Option<Property>> {
        Ok(self.properties()?.into_iter().find(|p| p.name() == name))
    }

    fn property_id(&self, name: &str) -> io::Result<Option<u32>> {
        Ok(self.property(name)?.map(|p| p.id()))
    }
}
