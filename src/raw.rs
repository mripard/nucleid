use std::{convert::TryInto, os::unix::io::AsRawFd};

use nix::{ioctl_readwrite, ioctl_write_ptr};

use crate::Result;

const DRM_IOCTL_BASE: u32 = 'd' as u32;
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

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct drm_mode_modeinfo {
    pub clock: u32,
    pub hdisplay: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub vscan: u16,
    pub vrefresh: u32,
    pub flags: u32,
    pub type_: u32,
    pub name: [u8; 32],
}

#[repr(C)]
pub struct drm_set_client_cap {
    pub capability: u64,
    pub value: u64,
}

ioctl_write_ptr!(
    drm_ioctl_set_client_cap,
    DRM_IOCTL_BASE,
    DRM_IOCTL_SET_CLIENT_CAP,
    drm_set_client_cap
);

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_card_res {
    pub fb_id_ptr: u64,
    pub crtc_id_ptr: u64,
    pub connector_id_ptr: u64,
    pub encoder_id_ptr: u64,
    pub count_fbs: u32,
    pub count_crtcs: u32,
    pub count_connectors: u32,
    pub count_encoders: u32,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}

ioctl_readwrite!(
    drm_ioctl_mode_getresources,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETRESOURCES,
    drm_mode_card_res
);

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_crtc {
    pub set_connectors_ptr: u64,
    pub count_connectors: u32,
    pub crtc_id: u32,
    pub fb_id: u32,
    pub x: u32,
    pub y: u32,
    pub gamma_size: u32,
    pub mode_valid: u32,
    pub mode: drm_mode_modeinfo,
}

ioctl_readwrite!(
    drm_ioctl_mode_getcrtc,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETCRTC,
    drm_mode_crtc
);

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_get_encoder {
    pub encoder_id: u32,
    pub encoder_type: u32,
    pub crtc_id: u32,
    pub possible_crtcs: u32,
    pub possible_clones: u32,
}

ioctl_readwrite!(
    drm_ioctl_mode_getencoder,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETENCODER,
    drm_mode_get_encoder
);

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_get_connector {
    pub encoders_ptr: u64,
    pub modes_ptr: u64,
    pub props_ptr: u64,
    pub prop_values_ptr: u64,
    pub count_modes: u32,
    pub count_props: u32,
    pub count_encoders: u32,
    pub encoder_id: u32,
    pub connector_id: u32,
    pub connector_type: u32,
    pub connector_type_id: u32,
    pub connection: u32,
    pub mm_width: u32,
    pub mm_height: u32,
    pub subpixel: u32,

    pub _pad: u32,
}

ioctl_readwrite!(
    drm_ioctl_mode_getconnector,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETCONNECTOR,
    drm_mode_get_connector
);

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_get_property {
    pub values_ptr: u64,
    pub enum_blob_ptr: u64,
    pub prop_id: u32,
    pub flags: u32,
    pub name: [u8; 32],
    pub count_values: u32,
    pub count_enum_blobs: u32,
}

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
    libc::c_uint
);

#[derive(Default)]
#[repr(C)]
pub struct drm_mode_crtc_page_flip {
    pub crtc_id: u32,
    pub fb_id: u32,
    pub flags: u32,
    pub reserved: u32,
    pub user_data: u64,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_create_dumb {
    pub height: u32,
    pub width: u32,
    pub bpp: u32,
    pub flags: u32,
    pub handle: u32,
    pub pitch: u32,
    pub size: u64,
}

ioctl_readwrite!(
    drm_ioctl_mode_create_dumb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_CREATE_DUMB,
    drm_mode_create_dumb
);

#[derive(Default)]
#[repr(C)]
pub struct drm_mode_map_dumb {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
}

ioctl_readwrite!(
    drm_ioctl_mode_map_dump,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_MAP_DUMB,
    drm_mode_map_dumb
);

#[derive(Default)]
#[repr(C)]
pub struct drm_mode_destroy_dumb {
    pub handle: u32,
}

ioctl_readwrite!(
    drm_ioctl_mode_destroy_dumb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_DESTROY_DUMB,
    drm_mode_destroy_dumb
);

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_get_plane_res {
    pub plane_id_ptr: u64,
    pub count_planes: u32,
}

ioctl_readwrite!(
    drm_ioctl_mode_getplaneresources,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANERESOURCES,
    drm_mode_get_plane_res
);

#[derive(Debug, Default)]
#[repr(C)]
pub struct drm_mode_get_plane {
    pub plane_id: u32,
    pub crtc_id: u32,
    pub fb_id: u32,
    pub possible_crtcs: u32,
    pub gamma_size: u32,
    pub count_format_types: u32,
    pub format_type_ptr: u64,
}

ioctl_readwrite!(
    drm_ioctl_mode_getplane,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANE,
    drm_mode_get_plane
);

#[derive(Default)]
#[repr(C)]
pub struct drm_mode_fb_cmd2 {
    pub fb_id: u32,
    pub width: u32,
    pub height: u32,
    pub pixel_format: u32,
    pub flags: u32,
    pub handles: [u32; 4],
    pub pitches: [u32; 4],
    pub offsets: [u32; 4],
    pub modifier: [u64; 4],
}

ioctl_readwrite!(
    drm_ioctl_mode_addfb2,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_ADDFB2,
    drm_mode_fb_cmd2
);

#[derive(Default)]
#[repr(C)]
pub struct drm_mode_obj_get_properties {
    props_ptr: u64,
    prop_values_ptr: u64,
    count_props: u32,
    obj_id: u32,
    obj_type: u32,
}

ioctl_readwrite!(
    drm_ioctl_mode_obj_getproperties,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_OBJ_GETPROPERTIES,
    drm_mode_obj_get_properties
);

#[derive(Default)]
#[repr(C)]
pub struct drm_mode_atomic {
    pub flags: u32,
    pub count_objs: u32,
    pub objs_ptr: u64,
    pub count_props_ptr: u64,
    pub props_ptr: u64,
    pub prop_values_ptr: u64,
    reserved: u64,
    pub user_data: u64,
}

ioctl_readwrite!(
    drm_ioctl_mode_atomic,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_ATOMIC,
    drm_mode_atomic
);

#[derive(Default)]
#[repr(C)]
pub struct drm_mode_create_blob {
    pub data: u64,
    pub length: u32,
    pub blob_id: u32,
}

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
