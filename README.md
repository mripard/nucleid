# Kernel Mode-Setting (KMS) abstraction crate

This library contains code to interact with a DRM/KMS device.

This library is a work in progress: expect missing features and breaking changes.

## Hello World

```no_run
extern crate nucleid;

use nucleid::Device;
 
fn main() {
    let device = Device::new("/dev/dri/card0")
        .unwrap();
}
```

## To-Do List

- [x] Migrate to nix
- [ ] Support Framebuffer format modifiers
- [ ] Support buffer import through dma-buf
- [ ] Review all the unwrap() calls to either get rid of them or annotate them
      with a comment
- [ ] Go through the doc and review the DRM vs KMS usage
- [ ] Get some larger review of the API
- [ ] Generate tests automatically to test the fourcc and DRM structure layouts
- [ ] Do some integration tests with VKMS
- [ ] Support test-only commits
- [ ] Support non-blocking commits
- [ ] Add some logging
- [ ] Do a C API?
