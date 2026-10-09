//! Use the device's SDL2 KMS/DRM implementation for Mali GBM scanout.
//! SDL owns modesetting, page flips and GBM buffer lifetimes.
use crate::{GfxError, Surface};
use libloading::Library;
use std::ffi::{c_char, c_void, CStr, CString};

type Ptr = *mut c_void;
type Init = unsafe extern "C" fn(u32) -> i32;
type Quit = unsafe extern "C" fn(u32);
type Error = unsafe extern "C" fn() -> *const c_char;
type Attr = unsafe extern "C" fn(i32, i32) -> i32;
type CreateWindow = unsafe extern "C" fn(*const c_char, i32, i32, i32, i32, u32) -> Ptr;
type DestroyWindow = unsafe extern "C" fn(Ptr);
type CreateContext = unsafe extern "C" fn(Ptr) -> Ptr;
type DeleteContext = unsafe extern "C" fn(Ptr);
type Current = unsafe extern "C" fn(Ptr, Ptr) -> i32;
type Interval = unsafe extern "C" fn(i32) -> i32;
type Swap = unsafe extern "C" fn(Ptr);
type Size = unsafe extern "C" fn(Ptr, *mut i32, *mut i32);
type Proc = unsafe extern "C" fn(*const c_char) -> *const c_void;

struct Sdl {
    init: Init,
    quit: Quit,
    error: Error,
    attr: Attr,
    create_window: CreateWindow,
    destroy_window: DestroyWindow,
    create_context: CreateContext,
    delete_context: DeleteContext,
    current: Current,
    interval: Interval,
    swap: Swap,
    size: Size,
    proc: Proc,
    _lib: Library,
}

impl Sdl {
    fn load() -> Result<Self, GfxError> {
        let lib = unsafe { Library::new("libSDL2-2.0.so.0") }
            .or_else(|_| unsafe { Library::new("libSDL2.so") })
            .map_err(|e| GfxError::Context(format!("SDL2: {e}")))?;
        unsafe fn symbol<T: Copy>(lib: &Library, name: &[u8]) -> Result<T, GfxError> {
            lib.get::<T>(name)
                .map(|s| *s)
                .map_err(|e| GfxError::Context(format!("SDL2 symbol: {e}")))
        }
        unsafe {
            Ok(Self {
                init: symbol(&lib, b"SDL_InitSubSystem\0")?,
                quit: symbol(&lib, b"SDL_QuitSubSystem\0")?,
                error: symbol(&lib, b"SDL_GetError\0")?,
                attr: symbol(&lib, b"SDL_GL_SetAttribute\0")?,
                create_window: symbol(&lib, b"SDL_CreateWindow\0")?,
                destroy_window: symbol(&lib, b"SDL_DestroyWindow\0")?,
                create_context: symbol(&lib, b"SDL_GL_CreateContext\0")?,
                delete_context: symbol(&lib, b"SDL_GL_DeleteContext\0")?,
                current: symbol(&lib, b"SDL_GL_MakeCurrent\0")?,
                interval: symbol(&lib, b"SDL_GL_SetSwapInterval\0")?,
                swap: symbol(&lib, b"SDL_GL_SwapWindow\0")?,
                size: symbol(&lib, b"SDL_GL_GetDrawableSize\0")?,
                proc: symbol(&lib, b"SDL_GL_GetProcAddress\0")?,
                _lib: lib,
            })
        }
    }
    fn fail(&self, what: &str) -> GfxError {
        let error = unsafe { (self.error)() };
        let text = if error.is_null() {
            "unknown error".into()
        } else {
            unsafe { CStr::from_ptr(error) }
                .to_string_lossy()
                .into_owned()
        };
        GfxError::Context(format!("{what}: {text}"))
    }
}

pub struct KmsdrmSurface {
    sdl: Sdl,
    window: Ptr,
    context: Ptr,
    size: (u32, u32),
}

impl KmsdrmSurface {
    pub fn new() -> Result<Self, GfxError> {
        // Explicitly choose the backend present in the dedicated G350 image.
        std::env::set_var("SDL_VIDEODRIVER", "KMSDRM");
        let sdl = Sdl::load()?;
        unsafe {
            if (sdl.init)(0x20) != 0 {
                return Err(sdl.fail("SDL KMS/DRM init"));
            }
            // SDL_GL_CONTEXT_PROFILE_MASK=21, ES=4, major=17, minor=18.
            for (attr, value) in [
                (21, 4),
                (17, 2),
                (18, 0),
                (0, 8),
                (1, 8),
                (2, 8),
                (3, 0),
                (5, 1),
            ] {
                if (sdl.attr)(attr, value) != 0 {
                    let error = sdl.fail("SDL GL attribute");
                    (sdl.quit)(0x20);
                    return Err(error);
                }
            }
            let title = CString::new("Slot G350").unwrap();
            // OPENGL | SHOWN | FULLSCREEN_DESKTOP; native panel is 640x480.
            let window = (sdl.create_window)(title.as_ptr(), 0, 0, 640, 480, 0x1007);
            if window.is_null() {
                let error = sdl.fail("SDL KMS/DRM window");
                (sdl.quit)(0x20);
                return Err(error);
            }
            let context = (sdl.create_context)(window);
            if context.is_null() {
                let error = sdl.fail("SDL GLES context");
                (sdl.destroy_window)(window);
                (sdl.quit)(0x20);
                return Err(error);
            }
            (sdl.interval)(1);
            let (mut width, mut height) = (0, 0);
            (sdl.size)(window, &mut width, &mut height);
            if width <= 0 || height <= 0 {
                (sdl.delete_context)(context);
                (sdl.destroy_window)(window);
                (sdl.quit)(0x20);
                return Err(GfxError::Context("SDL empty drawable".into()));
            }
            Ok(Self {
                sdl,
                window,
                context,
                size: (width as u32, height as u32),
            })
        }
    }
}

impl Surface for KmsdrmSurface {
    fn make_current(&mut self) -> Result<(), GfxError> {
        if unsafe { (self.sdl.current)(self.window, self.context) } != 0 {
            Err(self.sdl.fail("SDL make current"))
        } else {
            Ok(())
        }
    }
    fn window_size(&self) -> (u32, u32) {
        self.size
    }
    fn swap(&mut self) -> Result<(), GfxError> {
        unsafe {
            (self.sdl.swap)(self.window);
        }
        Ok(())
    }
    fn proc_address(&self, name: &str) -> *const c_void {
        let Ok(name) = CString::new(name) else {
            return std::ptr::null();
        };
        unsafe { (self.sdl.proc)(name.as_ptr()) }
    }
}
impl Drop for KmsdrmSurface {
    fn drop(&mut self) {
        unsafe {
            (self.sdl.delete_context)(self.context);
            (self.sdl.destroy_window)(self.window);
            (self.sdl.quit)(0x20);
        }
    }
}
