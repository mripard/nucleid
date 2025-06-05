use crate::{
    raw::{drm_mode_get_properties, drm_mode_object_type},
    Device, Property, Result,
};

pub trait Object {
    fn device(&self) -> Result<Device>;
    fn object_id(&self) -> u32;
    fn object_type(&self) -> drm_mode_object_type;

    fn properties(&self) -> Result<Vec<Property>> {
        let dev = self.device()?;
        let object_id = self.object_id();

        let properties = drm_mode_get_properties(&dev, self.object_type() as u32, object_id)?;

        let mut ret = Vec::new();
        for (prop_id, prop_value) in properties {
            let property = Property::new(&dev, object_id, prop_id, prop_value)?;

            ret.push(property);
        }

        Ok(ret)
    }

    fn property_id(&self, property: &str) -> Option<u32> {
        self.properties().map_or(None, |properties| {
            properties.into_iter().find_map(|prop| {
                if prop.name() == property {
                    Some(prop.id())
                } else {
                    None
                }
            })
        })
    }
}
