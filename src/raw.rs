use std::{convert::TryInto, ffi::c_uint, os::unix::io::AsRawFd};

use nix::{ioctl_readwrite, ioctl_write_ptr};
use num_enum::TryFromPrimitive;

use crate::{raw::bindgen::DRM_IOCTL_BASE, Result};

pub(crate) mod bindgen {
    #![allow(dead_code)]
    #![allow(missing_debug_implementations)]
    #![allow(missing_docs)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unsafe_code)]
    #![allow(unused_imports)]
    #![allow(clippy::struct_field_names)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub(crate) use bindgen::{
    drm_mode_atomic, drm_mode_card_res, drm_mode_create_blob, drm_mode_create_dumb, drm_mode_crtc,
    drm_mode_destroy_dumb, drm_mode_fb_cmd2, drm_mode_get_connector, drm_mode_get_encoder,
    drm_mode_get_plane, drm_mode_get_plane_res, drm_mode_get_property, drm_mode_map_dumb,
    drm_mode_modeinfo, drm_mode_obj_get_properties, drm_set_client_cap,
};

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u32)]
pub enum drm_mode_object_type {
    Any = bindgen::DRM_MODE_OBJECT_ANY,
    Property = bindgen::DRM_MODE_OBJECT_PROPERTY,
    Blob = bindgen::DRM_MODE_OBJECT_BLOB,
    Connector = bindgen::DRM_MODE_OBJECT_CONNECTOR,
    Crtc = bindgen::DRM_MODE_OBJECT_CRTC,
    Mode = bindgen::DRM_MODE_OBJECT_MODE,
    Encoder = bindgen::DRM_MODE_OBJECT_ENCODER,
    Plane = bindgen::DRM_MODE_OBJECT_PLANE,
    Fb = bindgen::DRM_MODE_OBJECT_FB,
}

/// Connector Status
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum drm_connector_status {
    /// This Connector is connected to a sink and can be enabled
    Connected = 1,

    /// This Connector hasn't detected a sink. Whether the Connector can be enabled or not is
    /// driver-dependant.
    Disconnected = 2,

    /// This Connector status couldn't reliably be determined. The Connector can be enabled
    /// with a fallback mode.
    Unknown = 3,
}

/// The Connector Type
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum drm_mode_connector_type {
    /// The Connector type couldn't be determined
    Unknown = bindgen::DRM_MODE_CONNECTOR_Unknown,

    /// A VGA DE-15 Connector
    VGA = bindgen::DRM_MODE_CONNECTOR_VGA,

    /// A DVI-I Connector
    DVII = bindgen::DRM_MODE_CONNECTOR_DVII,

    /// A DVI-D Connector
    DVID = bindgen::DRM_MODE_CONNECTOR_DVID,

    /// A DVI-A Connector
    DVIA = bindgen::DRM_MODE_CONNECTOR_DVIA,

    /// An RCA Connector carrying a CVBS signal
    Composite = bindgen::DRM_MODE_CONNECTOR_Composite,

    /// An S-Video Connector
    SVIDEO = bindgen::DRM_MODE_CONNECTOR_SVIDEO,

    /// An LVDS Connector
    LVDS = bindgen::DRM_MODE_CONNECTOR_LVDS,

    /// A Component Connector
    Component = bindgen::DRM_MODE_CONNECTOR_Component,

    /// A mini-Din-9 Connector
    MiniDin9 = bindgen::DRM_MODE_CONNECTOR_9PinDIN,

    /// A `DisplayPort` Connector
    DisplayPort = bindgen::DRM_MODE_CONNECTOR_DisplayPort,

    /// An HDMI-A Connector
    HDMIA = bindgen::DRM_MODE_CONNECTOR_HDMIA,

    /// An HDMI-B Connector
    HDMIB = bindgen::DRM_MODE_CONNECTOR_HDMIB,

    /// A TV Connector
    TV = bindgen::DRM_MODE_CONNECTOR_TV,

    /// An embedded `DisplayPort` Connector
    EDP = bindgen::DRM_MODE_CONNECTOR_eDP,

    /// A Virtual Connector
    Virtual = bindgen::DRM_MODE_CONNECTOR_VIRTUAL,

    /// A MIPI-DSI Connector
    DSI = bindgen::DRM_MODE_CONNECTOR_DSI,

    /// A MIPI-DPI Connector
    DPI = bindgen::DRM_MODE_CONNECTOR_DPI,

    /// A Writeback Connector
    Writeback = bindgen::DRM_MODE_CONNECTOR_WRITEBACK,

    /// An SPI-based Display Connector
    SPI = bindgen::DRM_MODE_CONNECTOR_SPI,
}

impl std::fmt::Display for drm_mode_connector_type {
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

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
#[allow(clippy::upper_case_acronyms)]
pub enum drm_mode_encoder_type {
    None = bindgen::DRM_MODE_ENCODER_NONE,
    DAC = bindgen::DRM_MODE_ENCODER_DAC,
    TMDS = bindgen::DRM_MODE_ENCODER_TMDS,
    LVDS = bindgen::DRM_MODE_ENCODER_LVDS,
    TVDAC = bindgen::DRM_MODE_ENCODER_TVDAC,
    Virtual = bindgen::DRM_MODE_ENCODER_VIRTUAL,
    DSI = bindgen::DRM_MODE_ENCODER_DSI,
    DPMST = bindgen::DRM_MODE_ENCODER_DPMST,
    DPI = bindgen::DRM_MODE_ENCODER_DPI,
}

const DRM_IOCTL_SET_CLIENT_CAP: u32 = 0x0d;
const DRM_IOCTL_MODE_GETRESOURCES: u32 = 0xa0;
const DRM_IOCTL_MODE_GETCRTC: u32 = 0xa1;
const DRM_IOCTL_MODE_GETENCODER: u32 = 0xa6;
const DRM_IOCTL_MODE_GETCONNECTOR: u32 = 0xa7;
const DRM_IOCTL_MODE_GETPROPERTY: u32 = 0xaa;
const DRM_IOCTL_MODE_RMFB: u32 = 0xaf;
const DRM_IOCTL_MODE_CREATE_DUMB: u32 = 0xb2;
const DRM_IOCTL_MODE_MAP_DUMB: u32 = 0xb3;
const DRM_IOCTL_MODE_DESTROY_DUMB: u32 = 0xb4;
const DRM_IOCTL_MODE_GETPLANERESOURCES: u32 = 0xb5;
const DRM_IOCTL_MODE_GETPLANE: u32 = 0xb6;
const DRM_IOCTL_MODE_ADDFB2: u32 = 0xb8;
const DRM_IOCTL_MODE_OBJ_GETPROPERTIES: u32 = 0xb9;
const DRM_IOCTL_MODE_ATOMIC: u32 = 0xbc;
const DRM_IOCTL_MODE_CREATEPROPBLOB: u32 = 0xbd;

ioctl_write_ptr!(
    drm_ioctl_set_client_cap,
    DRM_IOCTL_BASE,
    DRM_IOCTL_SET_CLIENT_CAP,
    drm_set_client_cap
);

ioctl_readwrite!(
    drm_ioctl_mode_getresources,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETRESOURCES,
    drm_mode_card_res
);

ioctl_readwrite!(
    drm_ioctl_mode_getcrtc,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETCRTC,
    drm_mode_crtc
);

ioctl_readwrite!(
    drm_ioctl_mode_getencoder,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETENCODER,
    drm_mode_get_encoder
);

ioctl_readwrite!(
    drm_ioctl_mode_getconnector,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETCONNECTOR,
    drm_mode_get_connector
);

ioctl_readwrite!(
    drm_ioctl_mode_getproperty,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPROPERTY,
    drm_mode_get_property
);

ioctl_readwrite!(
    drm_ioctl_mode_rmfb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_RMFB,
    c_uint
);

ioctl_readwrite!(
    drm_ioctl_mode_create_dumb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_CREATE_DUMB,
    drm_mode_create_dumb
);

ioctl_readwrite!(
    drm_ioctl_mode_map_dump,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_MAP_DUMB,
    drm_mode_map_dumb
);

ioctl_readwrite!(
    drm_ioctl_mode_destroy_dumb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_DESTROY_DUMB,
    drm_mode_destroy_dumb
);

ioctl_readwrite!(
    drm_ioctl_mode_getplaneresources,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANERESOURCES,
    drm_mode_get_plane_res
);

ioctl_readwrite!(
    drm_ioctl_mode_getplane,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANE,
    drm_mode_get_plane
);

ioctl_readwrite!(
    drm_ioctl_mode_addfb2,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_ADDFB2,
    drm_mode_fb_cmd2
);

ioctl_readwrite!(
    drm_ioctl_mode_obj_getproperties,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_OBJ_GETPROPERTIES,
    drm_mode_obj_get_properties
);

ioctl_readwrite!(
    drm_ioctl_mode_atomic,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_ATOMIC,
    drm_mode_atomic
);

ioctl_readwrite!(
    drm_ioctl_mode_createpropblob,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_CREATEPROPBLOB,
    drm_mode_create_blob
);

pub fn drm_mode_create_dumb_buffer(
    raw: &impl AsRawFd,
    width: u32,
    height: u32,
    bpp: u32,
) -> Result<drm_mode_create_dumb> {
    let fd = raw.as_raw_fd();

    let mut create = drm_mode_create_dumb {
        height,
        width,
        bpp,
        ..drm_mode_create_dumb::default()
    };

    unsafe { drm_ioctl_mode_create_dumb(fd, &mut create) }?;

    Ok(create)
}

pub fn drm_mode_add_framebuffer(
    raw: &impl AsRawFd,
    handle: u32,
    width: u32,
    pitch: u32,
    height: u32,
    fmt: u32,
) -> Result<u32> {
    let fd = raw.as_raw_fd();

    let mut fb = drm_mode_fb_cmd2 {
        width,
        height,
        pixel_format: fmt,
        ..drm_mode_fb_cmd2::default()
    };
    fb.handles[0] = handle;
    fb.pitches[0] = pitch;

    unsafe { drm_ioctl_mode_addfb2(fd, &mut fb) }?;

    Ok(fb.fb_id)
}

pub fn drm_mode_atomic_commit(
    raw: &impl AsRawFd,
    objs_ptr: &[u32],
    count_props_ptr: &[u32],
    props_ptr: &[u32],
    prop_values_ptr: &[u64],
) -> Result<()> {
    let fd = raw.as_raw_fd();

    let mut atomic: drm_mode_atomic = drm_mode_atomic {
        flags: 0x0400,
        count_objs: objs_ptr.len().try_into()?,
        objs_ptr: objs_ptr.as_ptr() as u64,
        count_props_ptr: count_props_ptr.as_ptr() as u64,
        props_ptr: props_ptr.as_ptr() as u64,
        prop_values_ptr: prop_values_ptr.as_ptr() as u64,
        reserved: 0,
        user_data: 0,
    };

    unsafe { drm_ioctl_mode_atomic(fd, &mut atomic) }?;

    Ok(())
}

pub fn drm_mode_create_property_blob<T: Sized>(raw: &impl AsRawFd, data: &T) -> Result<u32> {
    let fd = raw.as_raw_fd();

    let mut blob = drm_mode_create_blob {
        length: std::mem::size_of::<T>().try_into()?,
        data: std::ptr::from_ref::<T>(data) as u64,
        ..drm_mode_create_blob::default()
    };

    unsafe { drm_ioctl_mode_createpropblob(fd, &mut blob) }?;

    Ok(blob.blob_id)
}

pub fn drm_mode_remove_framebuffer(raw: &impl AsRawFd, id: u32) -> Result<()> {
    let fd = raw.as_raw_fd();
    let mut fb_id = id;

    unsafe { drm_ioctl_mode_rmfb(fd, &mut fb_id) }?;

    Ok(())
}

pub fn drm_mode_destroy_dumb_buffer(raw: &impl AsRawFd, handle: u32) -> Result<()> {
    let fd = raw.as_raw_fd();
    let mut destroy = drm_mode_destroy_dumb { handle };

    unsafe { drm_ioctl_mode_destroy_dumb(fd, &mut destroy) }?;

    Ok(())
}

pub fn drm_mode_get_encoder(raw: &impl AsRawFd, id: u32) -> Result<drm_mode_get_encoder> {
    let fd = raw.as_raw_fd();

    let mut encoder = drm_mode_get_encoder {
        encoder_id: id,
        ..drm_mode_get_encoder::default()
    };

    unsafe { drm_ioctl_mode_getencoder(fd, &mut encoder) }?;

    Ok(encoder)
}

pub fn drm_mode_get_connector(
    raw: &impl AsRawFd,
    id: u32,
    modes: Option<&mut Vec<drm_mode_modeinfo>>,
    encoders: Option<&mut Vec<u32>>,
) -> Result<drm_mode_get_connector> {
    let fd = raw.as_raw_fd();

    let mut count = drm_mode_get_connector {
        connector_id: id,
        ..drm_mode_get_connector::default()
    };

    unsafe { drm_ioctl_mode_getconnector(fd, &mut count) }?;

    if modes.is_none() && encoders.is_none() {
        return Ok(count);
    }

    let mut conn = drm_mode_get_connector {
        connector_id: id,
        ..drm_mode_get_connector::default()
    };

    if let Some(mod_info) = modes {
        mod_info.resize_with(count.count_modes as usize, Default::default);
        unsafe { mod_info.set_len(count.count_modes as usize) };
        conn.count_modes = count.count_modes;
        conn.modes_ptr = mod_info.as_mut_ptr() as u64;
    }

    if let Some(enc_ids) = encoders {
        enc_ids.resize_with(count.count_encoders as usize, Default::default);
        unsafe { enc_ids.set_len(count.count_encoders as usize) };
        conn.count_encoders = count.count_encoders;
        conn.encoders_ptr = enc_ids.as_mut_ptr() as u64;
    }

    unsafe { drm_ioctl_mode_getconnector(fd, &mut conn) }?;

    Ok(conn)
}

pub fn drm_mode_get_crtc(raw: &impl AsRawFd, id: u32) -> Result<drm_mode_crtc> {
    let fd = raw.as_raw_fd();

    let mut crtc = drm_mode_crtc {
        crtc_id: id,
        ..drm_mode_crtc::default()
    };

    unsafe { drm_ioctl_mode_getcrtc(fd, &mut crtc) }?;

    Ok(crtc)
}

pub fn drm_mode_get_plane(
    raw: &impl AsRawFd,
    id: u32,
    formats: Option<&mut Vec<u32>>,
) -> Result<drm_mode_get_plane> {
    let fd = raw.as_raw_fd();

    let mut count = drm_mode_get_plane {
        plane_id: id,
        ..drm_mode_get_plane::default()
    };

    unsafe { drm_ioctl_mode_getplane(fd, &mut count) }?;

    if let Some(formats) = formats {
        formats.resize_with(count.count_format_types as usize, Default::default);
        unsafe { formats.set_len(count.count_format_types as usize) };

        let mut plane = drm_mode_get_plane {
            plane_id: id,
            count_format_types: count.count_format_types,
            format_type_ptr: formats.as_mut_ptr() as u64,
            ..drm_mode_get_plane::default()
        };

        unsafe { drm_ioctl_mode_getplane(fd, &mut plane) }?;

        Ok(plane)
    } else {
        Ok(count)
    }
}

pub fn drm_mode_get_planes(raw: &impl AsRawFd) -> Result<Vec<u32>> {
    let fd = raw.as_raw_fd();

    let mut count = drm_mode_get_plane_res::default();

    unsafe { drm_ioctl_mode_getplaneresources(fd, &mut count) }?;

    let mut plane_ids: Vec<u32> = Vec::with_capacity(count.count_planes as usize);

    let mut resources = drm_mode_get_plane_res {
        count_planes: count.count_planes,
        plane_id_ptr: plane_ids.as_mut_ptr() as u64,
    };

    unsafe { drm_ioctl_mode_getplaneresources(fd, &mut resources) }?;

    unsafe { plane_ids.set_len(count.count_planes as usize) };

    Ok(plane_ids)
}

pub fn drm_mode_get_property(raw: &impl AsRawFd, id: u32) -> Result<drm_mode_get_property> {
    let fd = raw.as_raw_fd();

    let mut count = drm_mode_get_property {
        prop_id: id,
        ..drm_mode_get_property::default()
    };

    unsafe { drm_ioctl_mode_getproperty(fd, &mut count) }?;

    Ok(count)
}

pub fn drm_mode_get_properties(
    raw: &impl AsRawFd,
    object_type: u32,
    object_id: u32,
) -> Result<Vec<(u32, u64)>> {
    let fd = raw.as_raw_fd();

    let mut count = drm_mode_obj_get_properties {
        obj_type: object_type,
        obj_id: object_id,
        ..drm_mode_obj_get_properties::default()
    };

    unsafe { drm_ioctl_mode_obj_getproperties(fd, &mut count) }?;

    let mut prop_ids: Vec<u32> = Vec::with_capacity(count.count_props as usize);
    let mut prop_values: Vec<u64> = Vec::with_capacity(count.count_props as usize);

    let mut properties = drm_mode_obj_get_properties {
        obj_type: object_type,
        obj_id: object_id,
        count_props: count.count_props,
        props_ptr: prop_ids.as_mut_ptr() as u64,
        prop_values_ptr: prop_values.as_mut_ptr() as u64,
    };

    unsafe { drm_ioctl_mode_obj_getproperties(fd, &mut properties) }?;

    unsafe { prop_ids.set_len(count.count_props as usize) };
    unsafe { prop_values.set_len(count.count_props as usize) };

    Ok(prop_ids.into_iter().zip(prop_values).collect())
}

pub fn drm_mode_get_resources(
    raw: &impl AsRawFd,
    crtc_ids: Option<&mut Vec<u32>>,
    encoder_ids: Option<&mut Vec<u32>>,
    connector_ids: Option<&mut Vec<u32>>,
) -> Result<drm_mode_card_res> {
    let fd = raw.as_raw_fd();

    let mut count = drm_mode_card_res::default();

    unsafe { drm_ioctl_mode_getresources(fd, &mut count) }?;

    if crtc_ids.is_none() && encoder_ids.is_none() && connector_ids.is_none() {
        return Ok(count);
    }

    let mut resources = drm_mode_card_res::default();

    if let Some(crtcs) = crtc_ids {
        crtcs.resize_with(count.count_crtcs as usize, Default::default);
        unsafe { crtcs.set_len(count.count_crtcs as usize) };
        resources.count_crtcs = count.count_crtcs;
        resources.crtc_id_ptr = crtcs.as_mut_ptr() as u64;
    }

    if let Some(encoders) = encoder_ids {
        encoders.resize_with(count.count_encoders as usize, Default::default);
        unsafe { encoders.set_len(count.count_encoders as usize) };
        resources.count_encoders = count.count_encoders;
        resources.encoder_id_ptr = encoders.as_mut_ptr() as u64;
    }

    if let Some(connectors) = connector_ids {
        connectors.resize_with(count.count_connectors as usize, Default::default);
        unsafe { connectors.set_len(count.count_connectors as usize) };
        resources.count_connectors = count.count_connectors;
        resources.connector_id_ptr = connectors.as_mut_ptr() as u64;
    }

    unsafe { drm_ioctl_mode_getresources(fd, &mut resources) }?;

    Ok(resources)
}

pub fn drm_mode_map_dumb_buffer(raw: &impl AsRawFd, handle: u32) -> Result<drm_mode_map_dumb> {
    let fd = raw.as_raw_fd();

    let mut map = drm_mode_map_dumb {
        handle,
        ..drm_mode_map_dumb::default()
    };

    unsafe { drm_ioctl_mode_map_dump(fd, &mut map) }?;

    Ok(map)
}

pub fn drm_set_client_capability(raw: &impl AsRawFd, cap: u64) -> Result<()> {
    let fd = raw.as_raw_fd();
    let caps = drm_set_client_cap {
        capability: cap,
        value: 1,
    };

    unsafe { drm_ioctl_set_client_cap(fd, &caps) }?;

    Ok(())
}
