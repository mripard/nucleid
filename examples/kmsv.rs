use std::convert::TryInto;
use std::thread;
use std::time;

use anyhow::{Context, Result};

use clap::{Arg, ArgAction, Command};

use nucleid::{
    BufferType, ConnectorStatus, ConnectorUpdate, Device, Format, Framebuffer, ObjectUpdate,
    PlaneType, PlaneUpdate,
};

struct Image {
    buffer: Framebuffer,
    image_w: u32,
    image_h: u32,
    display_w: u16,
    display_h: u16,
    margin_w: u16,
    margin_h: u16,
}

fn main() -> Result<()> {
    let matches = Command::new("Kernel Mode Setting Image Viewer")
        .arg(
            Arg::new("device")
                .short('D')
                .help("DRM Device Path")
                .default_value("/dev/dri/card0"),
        )
        .arg(Arg::new("images").action(ArgAction::Append).required(true))
        .get_matches();
    let dev_path = matches.get_one::<String>("device").unwrap();
    let device = Device::new(dev_path).unwrap();
    let img_path = matches.get_many::<String>("images").unwrap();

    let connector = device
        .connectors()
        .find(|con| con.status().unwrap_or(ConnectorStatus::Unknown) == ConnectorStatus::Connected)
        .context("No Active Connector")?;

    let mode = connector
        .preferred_mode()
        .context("Couldn't find a mode for the connector")?;

    let output = device
        .output_from_connector(&connector)
        .context("Couldn't find a valid output for that connector")?;

    let plane = output
        .planes()
        .into_iter()
        .find(|plane| {
            plane.formats().any(|fmt| fmt == Format::XRGB8888)
                && plane.plane_type().unwrap() == PlaneType::Overlay
        })
        .context("Couldn't find a plane with the proper format")?;

    let images: Vec<Image> = img_path
        .map(|path| {
            let img = image::open(path).unwrap();
            let rgb_data = img.to_rgba8().into_vec();

            let img_h = img.height().try_into().unwrap();
            let img_w = img.width().try_into().unwrap();

            let mut buffer = device
                .allocate_buffer(BufferType::Dumb, img_w, img_h, 32)
                .unwrap()
                .into_framebuffer(Format::XRGB8888)
                .unwrap();

            let data = buffer.data();
            data.copy_from_slice(&rgb_data);

            let scale_h = f32::from(mode.height()) / img_h as f32;
            let scale_w = f32::from(mode.width()) / img_w as f32;
            let scale = scale_h
                .partial_cmp(&scale_w)
                .map(|ord| match ord {
                    std::cmp::Ordering::Less => scale_h,
                    _ => scale_w,
                })
                .unwrap_or(1.0);

            let image_h = img_h;
            let image_w = img_w;

            let display_h = ((img_h as f32) * scale).ceil() as u16;
            let display_w = ((img_w as f32) * scale).ceil() as u16;

            let margin_h = (mode.height() - display_h) / 2;
            let margin_w = (mode.width() - display_w) / 2;

            Image {
                buffer,
                image_h,
                image_w,
                display_h,
                display_w,
                margin_h,
                margin_w,
            }
        })
        .collect();

    let first = &images[0];
    let mut output = output
        .start_update()
        .set_mode(mode)
        .add_connector(
            ConnectorUpdate::new(&connector)
                .set_property("top margin", 0)
                .set_property("bottom margin", 0)
                .set_property("left margin", 0)
                .set_property("right margin", 0),
        )
        .add_plane(
            PlaneUpdate::new(&plane)
                .set_framebuffer(&first.buffer)
                .set_source_size(first.image_w as f32, first.image_h as f32)
                .set_source_coordinates(0.0, 0.0)
                .set_display_size(first.display_w.into(), first.display_h.into())
                .set_display_coordinates(first.margin_w.into(), first.margin_h.into()),
        )
        .commit()?;

    let mut index = 1;
    loop {
        let sleep = time::Duration::from_millis(1000);
        thread::sleep(sleep);

        let image = &images[index % images.len()];

        output = output
            .start_update()
            .add_plane(
                PlaneUpdate::new(&plane)
                    .set_framebuffer(&image.buffer)
                    .set_source_size(image.image_w as f32, image.image_h as f32)
                    .set_source_coordinates(0.0, 0.0)
                    .set_display_size(image.display_w.into(), image.display_h.into())
                    .set_display_coordinates(image.margin_w.into(), image.margin_h.into()),
            )
            .commit()?;

        index += 1;
    }
}
