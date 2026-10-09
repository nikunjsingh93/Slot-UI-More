//! Surfaceless Mesa context for Linux regression tests; not linked into device builds.
use crate::surface::{GfxError, Surface, OUT_H, OUT_W};
use std::ffi::{c_void, CString};
type Handle = *mut c_void;
#[link(name = "EGL")]
unsafe extern "C" {
    fn eglGetPlatformDisplay(platform: u32, native: Handle, attrs: *const isize) -> Handle;
    fn eglInitialize(display: Handle, major: *mut i32, minor: *mut i32) -> u32;
    fn eglBindAPI(api: u32) -> u32;
    fn eglChooseConfig(
        display: Handle,
        attrs: *const i32,
        configs: *mut Handle,
        n: i32,
        count: *mut i32,
    ) -> u32;
    fn eglCreatePbufferSurface(display: Handle, config: Handle, attrs: *const i32) -> Handle;
    fn eglCreateContext(
        display: Handle,
        config: Handle,
        share: Handle,
        attrs: *const i32,
    ) -> Handle;
    fn eglMakeCurrent(display: Handle, draw: Handle, read: Handle, context: Handle) -> u32;
    fn eglGetProcAddress(name: *const std::ffi::c_char) -> *const c_void;
    fn eglDestroyContext(display: Handle, context: Handle) -> u32;
    fn eglDestroySurface(display: Handle, surface: Handle) -> u32;
    fn eglTerminate(display: Handle) -> u32;
}
pub struct HeadlessSurface {
    display: Handle,
    surface: Handle,
    context: Handle,
}
impl HeadlessSurface {
    pub fn new() -> Result<Self, GfxError> {
        unsafe {
            let nil = std::ptr::null_mut();
            let display = eglGetPlatformDisplay(0x31dd, nil, std::ptr::null());
            if display.is_null() || eglInitialize(display, nil as *mut i32, nil as *mut i32) == 0 {
                return Err(GfxError::Context("surfaceless EGL unavailable".into()));
            }
            let es = std::env::var_os("SLOT_TEST_GLES").is_some();
            eglBindAPI(if es { 0x30a0 } else { 0x30a2 });
            let attrs = [
                0x3033,
                1,
                0x3040,
                if es { 4 } else { 8 },
                0x3024,
                8,
                0x3023,
                8,
                0x3022,
                8,
                0x3021,
                8,
                0x3038,
            ];
            let mut config = nil;
            let mut count = 0;
            if eglChooseConfig(display, attrs.as_ptr(), &mut config, 1, &mut count) == 0
                || count == 0
            {
                eglTerminate(display);
                return Err(GfxError::Context("EGL config unavailable".into()));
            }
            let size = [0x3057, OUT_W as i32, 0x3056, OUT_H as i32, 0x3038];
            let surface = eglCreatePbufferSurface(display, config, size.as_ptr());
            let ctx = if es {
                vec![0x3098, 2, 0x3038]
            } else {
                vec![0x3098, 3, 0x30fb, 3, 0x30fd, 1, 0x3038]
            };
            let context = eglCreateContext(display, config, nil, ctx.as_ptr());
            if surface.is_null()
                || context.is_null()
                || eglMakeCurrent(display, surface, surface, context) == 0
            {
                if !context.is_null() {
                    eglDestroyContext(display, context);
                }
                if !surface.is_null() {
                    eglDestroySurface(display, surface);
                }
                eglTerminate(display);
                return Err(GfxError::Context("EGL context unavailable".into()));
            }
            Ok(Self {
                display,
                surface,
                context,
            })
        }
    }
}
impl Surface for HeadlessSurface {
    fn make_current(&mut self) -> Result<(), GfxError> {
        if unsafe { eglMakeCurrent(self.display, self.surface, self.surface, self.context) } == 0 {
            Err(GfxError::Context("EGL make current failed".into()))
        } else {
            Ok(())
        }
    }
    fn window_size(&self) -> (u32, u32) {
        (OUT_W, OUT_H)
    }
    fn swap(&mut self) -> Result<(), GfxError> {
        unsafe { gl::Flush() };
        Ok(())
    }
    fn proc_address(&self, name: &str) -> *const c_void {
        CString::new(name).map_or(std::ptr::null(), |name| unsafe {
            eglGetProcAddress(name.as_ptr())
        })
    }
}
impl Drop for HeadlessSurface {
    fn drop(&mut self) {
        unsafe {
            eglMakeCurrent(
                self.display,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            eglDestroyContext(self.display, self.context);
            eglDestroySurface(self.display, self.surface);
            eglTerminate(self.display);
        }
    }
}
