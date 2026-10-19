# Shelf screenshots

These nine PNGs are captured from the repository's `slot-ui` shelf renderer, including its real cartridge/disc geometry, label clipping, drive/slot chrome and clock. They are frontend previews at the 720 × 480 reference canvas size, not physical-device photos or gameplay screenshots. Model panel fitting can differ.

All pictured game titles and label artwork are original documentation fixtures. No ROM, downloaded cover, private disc scan or save is required or published. The Game Boy Color example uses the supported rounded clear shell override. The images are documentation assets under the repository's license.

To regenerate in a Linux environment with the project's Rust toolchain, cached dependencies and Mesa/EGL development libraries:

```sh
cd slot
EGL_PLATFORM=surfaceless LIBGL_ALWAYS_SOFTWARE=1 cargo run --locked -p slot-ui --example readme_gallery -- ../docs/screenshots
```

The example generates temporary original label PNGs, captures all nine shelves, then removes those temporary labels. It does not open a game or touch an SD card.
