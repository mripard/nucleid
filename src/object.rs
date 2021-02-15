use crate::{raw::drm_mode_get_properties, Device, Property, Result};

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u32)]
pub enum Type {
    Any = 0,
    Property = 0xb0b0b0b0,
    Blob = 0xbbbbbbbb,
    Connector = 0xc0c0c0c0,
    Crtc = 0xcccccccc,
    Mode = 0xdededede,
    Encoder = 0xe0e0e0e0,
    Plane = 0xeeeeeeee,
    Fb = 0xfbfbfbfb,
}

pub trait Object {
    fn device(&self) -> Result<Device>;
    fn object_id(&self) -> u32;
    fn object_type(&self) -> Type;

    fn properties(&self) -> Result<Vec<Property>> {
        let dev = self.device()?;
        let object_id = self.object_id();

        let properties = drm_mode_get_properties(&dev, self.object_type() as u32, object_id)?;

        let mut ret = Vec::new();
        for (prop_id, prop_value) in properties {
            let property = Property::new(&dev, object_id, prop_id, prop_value)?;

            ret.push(property)
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
