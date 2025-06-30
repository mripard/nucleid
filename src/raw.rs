use std::{
    convert::{TryFrom, TryInto},
    ffi::c_uint,
    io,
    os::fd::{AsFd, BorrowedFd},
};

use facet::Facet;
use facet_enum_repr::FacetEnumRepr;
use rustix::{
    io::Errno,
    ioctl::{ioctl, opcode, Setter, Updater},
};

use crate::raw::bindgen::DRM_IOCTL_BASE;

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

#[repr(u32)]
#[derive(Clone, Copy, Debug, Facet, FacetEnumRepr)]
pub enum drm_mode_type {
    Builtin = bindgen::DRM_MODE_TYPE_BUILTIN,
    #[deprecated]
    ClockC = bindgen::DRM_MODE_TYPE_CLOCK_C,
    #[deprecated]
    CrtcC = bindgen::DRM_MODE_TYPE_CRTC_C,
    Preferred = bindgen::DRM_MODE_TYPE_PREFERRED,
    Default = bindgen::DRM_MODE_TYPE_DEFAULT,
    UserDef = bindgen:: DRM_MODE_TYPE_USERDEF,
    Driver = bindgen::DRM_MODE_TYPE_DRIVER,
}

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
#[derive(Clone, Copy, Debug, Facet, FacetEnumRepr, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, Facet, FacetEnumRepr, PartialEq)]
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

#[derive(Debug, Facet, FacetEnumRepr)]
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

const DRM_IOCTL_SET_CLIENT_CAP: u8 = 0x0d;
const DRM_IOCTL_MODE_GETRESOURCES: u8 = 0xa0;
const DRM_IOCTL_MODE_GETCRTC: u8 = 0xa1;
const DRM_IOCTL_MODE_GETENCODER: u8 = 0xa6;
const DRM_IOCTL_MODE_GETCONNECTOR: u8 = 0xa7;
const DRM_IOCTL_MODE_GETPROPERTY: u8 = 0xaa;
const DRM_IOCTL_MODE_RMFB: u8 = 0xaf;
const DRM_IOCTL_MODE_CREATE_DUMB: u8 = 0xb2;
const DRM_IOCTL_MODE_MAP_DUMB: u8 = 0xb3;
const DRM_IOCTL_MODE_DESTROY_DUMB: u8 = 0xb4;
const DRM_IOCTL_MODE_GETPLANERESOURCES: u8 = 0xb5;
const DRM_IOCTL_MODE_GETPLANE: u8 = 0xb6;
const DRM_IOCTL_MODE_ADDFB2: u8 = 0xb8;
const DRM_IOCTL_MODE_OBJ_GETPROPERTIES: u8 = 0xb9;
const DRM_IOCTL_MODE_ATOMIC: u8 = 0xbc;
const DRM_IOCTL_MODE_CREATEPROPBLOB: u8 = 0xbd;

macro_rules! ioctl_readwrite {
    ($name: ident, $base: expr, $nr: expr, $ty: ty, $doc: literal) => {
        #[doc = concat!($doc, "
# Errors

If there's an I/O Error while accessing the given file descriptor
")]
        pub fn $name(fd: BorrowedFd<'_>, mut arg: $ty) -> io::Result<$ty> {
            const OPCODE: u32 = opcode::read_write::<$ty>($base, $nr);

            // SAFETY: We checked both the opcode and the type.
            let ioctl_obj = unsafe { Updater::<OPCODE, $ty>::new(&mut arg) };

            // SAFETY: This function is unsafe because the driver isn't guaranteed to implement the
            // ioctl properly. We don't have much of a choice and still have to trust the
            // kernel there.
            unsafe { ioctl(fd, ioctl_obj) }
                .map(|()| arg)
                .map_err(<Errno as Into<io::Error>>::into)
        }
    };
}

const DRM_IOCTL_SET_CLIENT_CAP_OPCODE: u32 =
    opcode::write::<drm_set_client_cap>(DRM_IOCTL_BASE, DRM_IOCTL_SET_CLIENT_CAP);

/// Sets DRM Client Capabilities
///
/// # Errors
///
/// If there's an I/O Error while accessing the given file descriptor
pub fn drm_ioctl_set_client_cap(fd: BorrowedFd<'_>, cap: drm_set_client_cap) -> io::Result<()> {
    // SAFETY: We checked both the opcode and the type.
    let ioctl_obj =
        unsafe { Setter::<DRM_IOCTL_SET_CLIENT_CAP_OPCODE, drm_set_client_cap>::new(cap) };

    // SAFETY: This function is unsafe because the driver isn't guaranteed to implement the ioctl
    // properly. We don't have much of a choice and still have to trust the
    // kernel there.
    unsafe { ioctl(fd, ioctl_obj) }.map_err(<Errno as Into<io::Error>>::into)
}

ioctl_readwrite!(
    drm_ioctl_mode_getresources,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETRESOURCES,
    drm_mode_card_res,
    "Queries the device configuration"
);

ioctl_readwrite!(
    drm_ioctl_mode_getcrtc,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETCRTC,
    drm_mode_crtc,
    "Gets info for a given CRTC"
);

ioctl_readwrite!(
    drm_ioctl_mode_getencoder,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETENCODER,
    drm_mode_get_encoder,
    "Gets info for a given encoder"
);

ioctl_readwrite!(
    drm_ioctl_mode_getconnector,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETCONNECTOR,
    drm_mode_get_connector,
    "Gets info for a given connector"
);

ioctl_readwrite!(
    drm_ioctl_mode_getproperty,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPROPERTY,
    drm_mode_get_property,
    "Gets info for a given property"
);

ioctl_readwrite!(
    drm_ioctl_mode_rmfb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_RMFB,
    c_uint,
    "Removes a framebuffer object"
);

ioctl_readwrite!(
    drm_ioctl_mode_create_dumb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_CREATE_DUMB,
    drm_mode_create_dumb,
    "Creates a new dumb buffer object"
);

ioctl_readwrite!(
    drm_ioctl_mode_map_dumb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_MAP_DUMB,
    drm_mode_map_dumb,
    "Maps a new dumb buffer object"
);

ioctl_readwrite!(
    drm_ioctl_mode_destroy_dumb,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_DESTROY_DUMB,
    drm_mode_destroy_dumb,
    "Destroys a new dumb buffer object"
);

ioctl_readwrite!(
    drm_ioctl_mode_getplaneresources,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANERESOURCES,
    drm_mode_get_plane_res,
    "Enumerates planes"
);

ioctl_readwrite!(
    drm_ioctl_mode_getplane,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANE,
    drm_mode_get_plane,
    "Gets info for a given plane"
);

ioctl_readwrite!(
    drm_ioctl_mode_addfb2,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_ADDFB2,
    drm_mode_fb_cmd2,
    "Adds a framebuffer object"
);

ioctl_readwrite!(
    drm_ioctl_mode_obj_getproperties,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_OBJ_GETPROPERTIES,
    drm_mode_obj_get_properties,
    "Lists properties attached to an object"
);

ioctl_readwrite!(
    drm_ioctl_mode_atomic,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_ATOMIC,
    drm_mode_atomic,
    "Performs an atomic commit"
);

ioctl_readwrite!(
    drm_ioctl_mode_createpropblob,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_CREATEPROPBLOB,
    drm_mode_create_blob,
    "Creates a blob value"
);

pub fn drm_mode_create_dumb_buffer(
    raw: &impl AsFd,
    width: u32,
    height: u32,
    bpp: u32,
) -> io::Result<drm_mode_create_dumb> {
    drm_ioctl_mode_create_dumb(
        raw.as_fd(),
        drm_mode_create_dumb {
            height,
            width,
            bpp,
            ..drm_mode_create_dumb::default()
        },
    )
}

pub fn drm_mode_add_framebuffer(
    raw: &impl AsFd,
    handle: u32,
    width: u32,
    pitch: u32,
    height: u32,
    fmt: u32,
) -> io::Result<u32> {
    let mut fb = drm_mode_fb_cmd2 {
        width,
        height,
        pixel_format: fmt,
        ..drm_mode_fb_cmd2::default()
    };
    fb.handles[0] = handle;
    fb.pitches[0] = pitch;

    drm_ioctl_mode_addfb2(raw.as_fd(), fb).map(|fb| fb.fb_id)
}

pub fn drm_mode_atomic_commit(
    raw: &impl AsFd,
    objs_ptr: &[u32],
    count_props_ptr: &[u32],
    props_ptr: &[u32],
    prop_values_ptr: &[u64],
) -> io::Result<()> {
    drm_ioctl_mode_atomic(
        raw.as_fd(),
        drm_mode_atomic {
            flags: 0x0400,
            count_objs: objs_ptr.len().try_into().map_err(|_e| {
                io::Error::new(
                    io::ErrorKind::ArgumentListTooLong,
                    "Too many objects passed",
                )
            })?,
            objs_ptr: objs_ptr.as_ptr() as u64,
            count_props_ptr: count_props_ptr.as_ptr() as u64,
            props_ptr: props_ptr.as_ptr() as u64,
            prop_values_ptr: prop_values_ptr.as_ptr() as u64,
            reserved: 0,
            user_data: 0,
        },
    )
    .map(|_v| ())
}

pub fn drm_mode_create_property_blob<T: Sized>(raw: &impl AsFd, data: &T) -> io::Result<u32> {
    drm_ioctl_mode_createpropblob(
        raw.as_fd(),
        drm_mode_create_blob {
            length: std::mem::size_of::<T>()
                .try_into()
                .map_err(|_e| io::Error::new(io::ErrorKind::InvalidInput, "Blob is too large"))?,
            data: std::ptr::from_ref::<T>(data) as u64,
            ..drm_mode_create_blob::default()
        },
    )
    .map(|blob| blob.blob_id)
}

pub fn drm_mode_remove_framebuffer(raw: &impl AsFd, id: u32) -> io::Result<()> {
    drm_ioctl_mode_rmfb(raw.as_fd(), id).map(|_v| ())
}

pub fn drm_mode_destroy_dumb_buffer(raw: &impl AsFd, handle: u32) -> io::Result<()> {
    drm_ioctl_mode_destroy_dumb(raw.as_fd(), drm_mode_destroy_dumb { handle }).map(|_v| ())
}

pub fn drm_mode_get_encoder(raw: &impl AsFd, id: u32) -> io::Result<drm_mode_get_encoder> {
    drm_ioctl_mode_getencoder(
        raw.as_fd(),
        drm_mode_get_encoder {
            encoder_id: id,
            ..drm_mode_get_encoder::default()
        },
    )
}

pub fn drm_mode_get_connector(
    raw: &impl AsFd,
    id: u32,
    modes: Option<&mut Vec<drm_mode_modeinfo>>,
    encoders: Option<&mut Vec<u32>>,
) -> io::Result<drm_mode_get_connector> {
    let fd = raw.as_fd();

    let count = drm_ioctl_mode_getconnector(
        fd,
        drm_mode_get_connector {
            connector_id: id,
            ..drm_mode_get_connector::default()
        },
    )?;

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

    drm_ioctl_mode_getconnector(fd, conn)
}

pub fn drm_mode_get_crtc(raw: &impl AsFd, id: u32) -> io::Result<drm_mode_crtc> {
    drm_ioctl_mode_getcrtc(
        raw.as_fd(),
        drm_mode_crtc {
            crtc_id: id,
            ..drm_mode_crtc::default()
        },
    )
}

pub fn drm_mode_get_plane(
    raw: &impl AsFd,
    id: u32,
    formats: Option<&mut Vec<u32>>,
) -> io::Result<drm_mode_get_plane> {
    let fd = raw.as_fd();

    let count = drm_ioctl_mode_getplane(
        fd,
        drm_mode_get_plane {
            plane_id: id,
            ..drm_mode_get_plane::default()
        },
    )?;

    if let Some(formats) = formats {
        formats.resize_with(count.count_format_types as usize, Default::default);
        unsafe { formats.set_len(count.count_format_types as usize) };

        drm_ioctl_mode_getplane(
            fd,
            drm_mode_get_plane {
                plane_id: id,
                count_format_types: count.count_format_types,
                format_type_ptr: formats.as_mut_ptr() as u64,
                ..drm_mode_get_plane::default()
            },
        )
    } else {
        Ok(count)
    }
}

pub fn drm_mode_get_planes(raw: &impl AsFd) -> io::Result<Vec<u32>> {
    let fd = raw.as_fd();

    let count = drm_ioctl_mode_getplaneresources(fd, drm_mode_get_plane_res::default())?;

    let mut plane_ids: Vec<u32> = Vec::with_capacity(count.count_planes as usize);

    drm_ioctl_mode_getplaneresources(
        fd,
        drm_mode_get_plane_res {
            count_planes: count.count_planes,
            plane_id_ptr: plane_ids.as_mut_ptr() as u64,
        },
    )?;

    unsafe { plane_ids.set_len(count.count_planes as usize) };

    Ok(plane_ids)
}

pub fn drm_mode_get_property(raw: &impl AsFd, id: u32) -> io::Result<drm_mode_get_property> {
    drm_ioctl_mode_getproperty(
        raw.as_fd(),
        drm_mode_get_property {
            prop_id: id,
            ..drm_mode_get_property::default()
        },
    )
}

pub fn drm_mode_get_properties(
    raw: &impl AsFd,
    object_type: u32,
    object_id: u32,
) -> io::Result<Vec<(u32, u64)>> {
    let fd = raw.as_fd();

    let count = drm_ioctl_mode_obj_getproperties(
        fd,
        drm_mode_obj_get_properties {
            obj_type: object_type,
            obj_id: object_id,
            ..drm_mode_obj_get_properties::default()
        },
    )?;

    let mut prop_ids: Vec<u32> = Vec::with_capacity(count.count_props as usize);
    let mut prop_values: Vec<u64> = Vec::with_capacity(count.count_props as usize);

    drm_ioctl_mode_obj_getproperties(
        fd,
        drm_mode_obj_get_properties {
            obj_type: object_type,
            obj_id: object_id,
            count_props: count.count_props,
            props_ptr: prop_ids.as_mut_ptr() as u64,
            prop_values_ptr: prop_values.as_mut_ptr() as u64,
        },
    )?;

    unsafe { prop_ids.set_len(count.count_props as usize) };
    unsafe { prop_values.set_len(count.count_props as usize) };

    Ok(prop_ids.into_iter().zip(prop_values).collect())
}

pub fn drm_mode_get_resources(
    raw: &impl AsFd,
    crtc_ids: Option<&mut Vec<u32>>,
    encoder_ids: Option<&mut Vec<u32>>,
    connector_ids: Option<&mut Vec<u32>>,
) -> io::Result<drm_mode_card_res> {
    let fd = raw.as_fd();

    let count = drm_ioctl_mode_getresources(fd, drm_mode_card_res::default())?;

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

    drm_ioctl_mode_getresources(fd, resources)
}

pub fn drm_mode_map_dumb_buffer(raw: &impl AsFd, handle: u32) -> io::Result<drm_mode_map_dumb> {
    drm_ioctl_mode_map_dumb(
        raw.as_fd(),
        drm_mode_map_dumb {
            handle,
            ..drm_mode_map_dumb::default()
        },
    )
}

pub fn drm_set_client_capability(raw: &impl AsFd, cap: u64) -> io::Result<()> {
    drm_ioctl_set_client_cap(
        raw.as_fd(),
        drm_set_client_cap {
            capability: cap,
            value: 1,
        },
    )
}
