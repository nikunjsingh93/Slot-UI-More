//! A core-owned GLES context on the existing display. The UI retains its own context.
//! CPU readback at native N64 resolution avoids cross-thread GL ownership or DRM takeover.
use crate::core::CoreError;
use libloading::Library;
use std::ffi::{c_char, c_void, CStr};
type H = *mut c_void;
type GetProc = unsafe extern "C" fn(*const c_char) -> *const c_void;
type Current = unsafe extern "C" fn() -> H;
type Choose = unsafe extern "C" fn(H, *const i32, *mut H, i32, *mut i32) -> u32;
type Pbuffer = unsafe extern "C" fn(H, H, *const i32) -> H;
type Context = unsafe extern "C" fn(H, H, H, *const i32) -> H;
type Make = unsafe extern "C" fn(H, H, H, H) -> u32;
type Destroy = unsafe extern "C" fn(H, H) -> u32;
type BindApi = unsafe extern "C" fn(u32) -> u32;
type Gen = unsafe extern "C" fn(i32, *mut u32);
type Bind = unsafe extern "C" fn(u32, u32);
type TexImage = unsafe extern "C" fn(u32, i32, i32, i32, i32, i32, u32, u32, *const c_void);
type TexParam = unsafe extern "C" fn(u32, u32, i32);
type AttachTex = unsafe extern "C" fn(u32, u32, u32, u32, i32);
type Storage = unsafe extern "C" fn(u32, u32, i32, i32);
type AttachRb = unsafe extern "C" fn(u32, u32, u32, u32);
type Check = unsafe extern "C" fn(u32) -> u32;
type Read = unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, *mut c_void);
type GetInt = unsafe extern "C" fn(u32, *mut i32);
type Delete = unsafe extern "C" fn(i32, *const u32);

pub struct Display {
    display: usize,
    egl: Library,
}
// Only the EGLDisplay handle is captured on the UI thread. Its display outlives the core.
impl Display {
    pub fn capture() -> Option<Self> {
        unsafe {
            let egl = Library::new("libEGL.so.1")
                .or_else(|_| Library::new("libEGL.so"))
                .ok()?;
            let get = *egl.get::<Current>(b"eglGetCurrentDisplay\0").ok()?;
            let display = get() as usize;
            if display == 0 {
                return None;
            }
            Some(Self { display, egl })
        }
    }
    pub fn proc(&self, name: &CStr) -> *const c_void {
        unsafe {
            self.egl
                .get::<GetProc>(b"eglGetProcAddress\0")
                .map_or(std::ptr::null(), |get| get(name.as_ptr()))
        }
    }
    pub fn open(
        &self,
        version: u32,
        depth: bool,
        stencil: bool,
        width: u32,
        height: u32,
    ) -> Result<Hardware, CoreError> {
        if width == 0 || height == 0 || width > 1024 || height > 1024 {
            return Err(CoreError::Load(
                "invalid hardware framebuffer dimensions".into(),
            ));
        }
        unsafe {
            macro_rules! e {
                ($name:literal,$ty:ty) => {
                    *self
                        .egl
                        .get::<$ty>(concat!($name, "\0").as_bytes())
                        .map_err(|x| CoreError::Load(x.to_string()))?
                };
            }
            let choose = e!("eglChooseConfig", Choose);
            let create = e!("eglCreateContext", Context);
            let pbuffer = e!("eglCreatePbufferSurface", Pbuffer);
            let make = e!("eglMakeCurrent", Make);
            let destroy_ctx = e!("eglDestroyContext", Destroy);
            let destroy_surface = e!("eglDestroySurface", Destroy);
            let bind = e!("eglBindAPI", BindApi);
            let display = self.display as H;
            if bind(0x30a0) == 0 {
                return Err(CoreError::Load("N64: cannot bind GLES".into()));
            }
            let attrs = [
                0x3033,
                1,
                0x3040,
                if version >= 3 { 0x40 } else { 4 },
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
            let mut config = std::ptr::null_mut();
            let mut count = 0;
            if choose(display, attrs.as_ptr(), &mut config, 1, &mut count) == 0 || count == 0 {
                return Err(CoreError::Load(
                    "N64: GLES pbuffer config unavailable".into(),
                ));
            }
            let size = [0x3057, 1, 0x3056, 1, 0x3038];
            let surface = pbuffer(display, config, size.as_ptr());
            if surface.is_null() {
                return Err(CoreError::Load("N64: offscreen surface unavailable".into()));
            }
            let ctxattrs = [0x3098, version as i32, 0x3038];
            let context = create(display, config, std::ptr::null_mut(), ctxattrs.as_ptr());
            if context.is_null() || make(display, surface, surface, context) == 0 {
                if !context.is_null() {
                    destroy_ctx(display, context);
                }
                destroy_surface(display, surface);
                return Err(CoreError::Load("N64: GLES context creation failed".into()));
            }
            let gl = match Gl::load(self) {
                Ok(gl) => gl,
                Err(e) => {
                    make(
                        display,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );
                    destroy_ctx(display, context);
                    destroy_surface(display, surface);
                    return Err(e);
                }
            };
            let mut hw = Hardware {
                display: self.display,
                surface: surface as usize,
                context: context as usize,
                make,
                destroy_ctx,
                destroy_surface,
                gl,
                fbo: 0,
                tex: 0,
                depth: 0,
                stencil: 0,
                width,
                height,
            };
            (hw.gl.gen_fbo)(1, &mut hw.fbo);
            (hw.gl.bind_fbo)(0x8d40, hw.fbo);
            (hw.gl.gen_tex)(1, &mut hw.tex);
            (hw.gl.bind_tex)(0x0de1, hw.tex);
            (hw.gl.tex_image)(
                0x0de1,
                0,
                0x1908,
                width as i32,
                height as i32,
                0,
                0x1908,
                0x1401,
                std::ptr::null(),
            );
            (hw.gl.tex_param)(0x0de1, 0x2801, 0x2600);
            (hw.gl.tex_param)(0x0de1, 0x2800, 0x2600);
            (hw.gl.attach_tex)(0x8d40, 0x8ce0, 0x0de1, hw.tex, 0);
            if depth {
                (hw.gl.gen_rb)(1, &mut hw.depth);
                (hw.gl.bind_rb)(0x8d41, hw.depth);
                (hw.gl.storage)(0x8d41, 0x81a5, width as i32, height as i32);
                (hw.gl.attach_rb)(0x8d40, 0x8d00, 0x8d41, hw.depth);
            }
            if stencil {
                (hw.gl.gen_rb)(1, &mut hw.stencil);
                (hw.gl.bind_rb)(0x8d41, hw.stencil);
                (hw.gl.storage)(0x8d41, 0x8d48, width as i32, height as i32);
                (hw.gl.attach_rb)(0x8d40, 0x8d20, 0x8d41, hw.stencil);
            }
            if (hw.gl.check)(0x8d40) != 0x8cd5 {
                return Err(CoreError::Load(
                    "N64: incomplete hardware framebuffer".into(),
                ));
            }
            Ok(hw)
        }
    }
}
struct Gl {
    gen_fbo: Gen,
    bind_fbo: Bind,
    gen_tex: Gen,
    bind_tex: Bind,
    tex_image: TexImage,
    tex_param: TexParam,
    attach_tex: AttachTex,
    gen_rb: Gen,
    bind_rb: Bind,
    storage: Storage,
    attach_rb: AttachRb,
    check: Check,
    read: Read,
    get_int: GetInt,
    delete_fbo: Delete,
    delete_tex: Delete,
    delete_rb: Delete,
}
impl Gl {
    unsafe fn load(d: &Display) -> Result<Self, CoreError> {
        macro_rules! g {
            ($name:literal,$ty:ty) => {{
                let p = d.proc(CStr::from_bytes_with_nul_unchecked(
                    concat!($name, "\0").as_bytes(),
                ));
                if p.is_null() {
                    return Err(CoreError::Load(
                        concat!("N64 missing GLES function ", $name).into(),
                    ));
                }
                std::mem::transmute::<*const c_void, $ty>(p)
            }};
        }
        Ok(Self {
            gen_fbo: g!("glGenFramebuffers", Gen),
            bind_fbo: g!("glBindFramebuffer", Bind),
            gen_tex: g!("glGenTextures", Gen),
            bind_tex: g!("glBindTexture", Bind),
            tex_image: g!("glTexImage2D", TexImage),
            tex_param: g!("glTexParameteri", TexParam),
            attach_tex: g!("glFramebufferTexture2D", AttachTex),
            gen_rb: g!("glGenRenderbuffers", Gen),
            bind_rb: g!("glBindRenderbuffer", Bind),
            storage: g!("glRenderbufferStorage", Storage),
            attach_rb: g!("glFramebufferRenderbuffer", AttachRb),
            check: g!("glCheckFramebufferStatus", Check),
            read: g!("glReadPixels", Read),
            get_int: g!("glGetIntegerv", GetInt),
            delete_fbo: g!("glDeleteFramebuffers", Delete),
            delete_tex: g!("glDeleteTextures", Delete),
            delete_rb: g!("glDeleteRenderbuffers", Delete),
        })
    }
}
pub struct Hardware {
    display: usize,
    surface: usize,
    context: usize,
    make: Make,
    destroy_ctx: Destroy,
    destroy_surface: Destroy,
    gl: Gl,
    pub fbo: u32,
    tex: u32,
    depth: u32,
    stencil: u32,
    width: u32,
    height: u32,
}
impl Hardware {
    pub fn current(&self) -> bool {
        unsafe {
            (self.make)(
                self.display as H,
                self.surface as H,
                self.surface as H,
                self.context as H,
            ) != 0
        }
    }
    pub fn read(&self, w: u32, h: u32, bottom_left: bool) -> Option<Vec<u8>> {
        let mut data = Vec::new();
        self.read_into(w, h, bottom_left, &mut data).then_some(data)
    }
    pub fn read_into(&self, w: u32, h: u32, bottom_left: bool, data: &mut Vec<u8>) -> bool {
        if w == 0 || h == 0 || w > self.width || h > self.height {
            return false;
        }
        data.resize((w * h * 4) as usize, 0);
        unsafe {
            let mut previous = 0;
            (self.gl.get_int)(0x8ca6, &mut previous);
            (self.gl.bind_fbo)(0x8d40, self.fbo);
            (self.gl.read)(
                0,
                0,
                w as i32,
                h as i32,
                0x1908,
                0x1401,
                data.as_mut_ptr() as H,
            );
            (self.gl.bind_fbo)(0x8d40, previous as u32);
        }
        rgba_to_xrgb(data, w as usize, h as usize, bottom_left);
        true
    }
}
impl Drop for Hardware {
    fn drop(&mut self) {
        unsafe {
            self.current();
            (self.gl.delete_fbo)(1, &self.fbo);
            (self.gl.delete_tex)(1, &self.tex);
            if self.depth != 0 {
                (self.gl.delete_rb)(1, &self.depth);
            }
            if self.stencil != 0 {
                (self.gl.delete_rb)(1, &self.stencil);
            }
            (self.make)(
                self.display as H,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            (self.destroy_ctx)(self.display as H, self.context as H);
            (self.destroy_surface)(self.display as H, self.surface as H);
        }
    }
}

// Swizzle and flip in the existing allocation, rather than allocating and
// copying two images before the frontend enlarges them again.
fn rgba_to_xrgb(data: &mut [u8], w: usize, h: usize, bottom_left: bool) {
    for px in data.chunks_exact_mut(4) {
        px.swap(0, 2);
        px[3] = 0;
    }
    if bottom_left {
        let stride = w * 4;
        for y in 0..h / 2 {
            let other = (h - 1 - y) * stride;
            let (upper, lower) = data.split_at_mut(other);
            upper[y * stride..(y + 1) * stride].swap_with_slice(&mut lower[..stride]);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::rgba_to_xrgb;
    #[test]
    fn readback_swizzles_and_flips_odd_height_without_moving_middle_row() {
        let mut pixels = vec![1, 2, 3, 255, 4, 5, 6, 255, 7, 8, 9, 255];
        rgba_to_xrgb(&mut pixels, 1, 3, true);
        assert_eq!(pixels, [9, 8, 7, 0, 6, 5, 4, 0, 3, 2, 1, 0]);
    }
    #[test]
    fn top_left_readback_keeps_row_order() {
        let mut pixels = vec![1, 2, 3, 255, 4, 5, 6, 255];
        rgba_to_xrgb(&mut pixels, 1, 2, false);
        assert_eq!(pixels, [3, 2, 1, 0, 6, 5, 4, 0]);
    }
}
