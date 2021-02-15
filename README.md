# Kernel Mode-Setting (KMS) abstraction crate

This library contains code to interact with a DRM/KMS device.
 
This library is a work in progress: expect missing features and breaking changes.

# Hello World

```no_run
extern crate nucleid;

use nucleid::Device;
 
fn main() {
    let device = Device::new("/dev/dri/card0")
        .unwrap();
}
```