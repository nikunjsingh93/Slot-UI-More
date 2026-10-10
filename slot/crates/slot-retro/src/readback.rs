//! Bounded, nonblocking GLES3 readback for Dreamcast's offscreen renderer.
//! Three pixel-pack buffers keep emulation off the GPU completion wait. N64
//! and PSP continue to use their existing synchronous path.
use crate::hardware::Display;
use std::ffi::{c_void, CStr};

const PACK: u32 = 0x88eb;
type Gen = unsafe extern "C" fn(i32, *mut u32);
type Bind = unsafe extern "C" fn(u32, u32);
type Data = unsafe extern "C" fn(u32, isize, *const c_void, u32);
type Map = unsafe extern "C" fn(u32, isize, isize, u32) -> *const c_void;
type Unmap = unsafe extern "C" fn(u32) -> u8;
type Fence = unsafe extern "C" fn(u32, u32) -> *mut c_void;
type Wait = unsafe extern "C" fn(*mut c_void, u32, u64) -> u32;
type DeleteSync = unsafe extern "C" fn(*mut c_void);
type Delete = unsafe extern "C" fn(i32, *const u32);
type Flush = unsafe extern "C" fn();
type Read = unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, *mut c_void);
type GetInt = unsafe extern "C" fn(u32, *mut i32);

struct Slot {
    buffer: u32,
    fence: usize,
    sequence: u64,
}

pub(crate) struct Readback {
    slots: [Slot; 3],
    bytes: usize,
    shape: (u32, u32),
    sequence: u64,
    delivered: u64,
    gen: Gen,
    bind: Bind,
    data: Data,
    map: Map,
    unmap: Unmap,
    fence: Fence,
    wait: Wait,
    delete_sync: DeleteSync,
    delete: Delete,
    flush: Flush,
    read: Read,
    get_int: GetInt,
}

impl Readback {
    pub(crate) unsafe fn new(display: &Display) -> Option<Self> {
        macro_rules! load {
            ($name:literal, $ty:ty) => {{
                let name = CStr::from_bytes_with_nul_unchecked(concat!($name, "\0").as_bytes());
                let address = display.proc(name);
                if address.is_null() {
                    return None;
                }
                std::mem::transmute::<*const c_void, $ty>(address)
            }};
        }
        // Flycast asks for an ES2-compatible context even when the device
        // supplies GLES3. Check the actual GL version, not the requested enum.
        let get_string = load!("glGetString", unsafe extern "C" fn(u32) -> *const u8);
        let version = get_string(0x1f02);
        if version.is_null() {
            return None;
        }
        let version = CStr::from_ptr(version as *const std::ffi::c_char).to_string_lossy();
        if !(version.starts_with("OpenGL ES 3.") || version.starts_with("OpenGL ES 4.")) {
            return None;
        }
        Some(Self {
            slots: std::array::from_fn(|_| Slot {
                buffer: 0,
                fence: 0,
                sequence: 0,
            }),
            bytes: 0,
            shape: (0, 0),
            sequence: 0,
            delivered: 0,
            gen: load!("glGenBuffers", Gen),
            bind: load!("glBindBuffer", Bind),
            data: load!("glBufferData", Data),
            map: load!("glMapBufferRange", Map),
            unmap: load!("glUnmapBuffer", Unmap),
            fence: load!("glFenceSync", Fence),
            wait: load!("glClientWaitSync", Wait),
            delete_sync: load!("glDeleteSync", DeleteSync),
            delete: load!("glDeleteBuffers", Delete),
            flush: load!("glFlush", Flush),
            read: load!("glReadPixels", Read),
            get_int: load!("glGetIntegerv", GetInt),
        })
    }

    /// None signals a driver failure; Some(false) means no completed frame yet.
    /// Never waits for a fence and never maps a still-busy buffer.
    pub(crate) unsafe fn capture(&mut self, w: u32, h: u32, output: &mut Vec<u8>) -> Option<bool> {
        let mut previous = 0;
        (self.get_int)(0x88ed, &mut previous);
        let result = self.capture_bound(w, h, output);
        (self.bind)(PACK, previous as u32);
        result
    }

    unsafe fn capture_bound(&mut self, w: u32, h: u32, output: &mut Vec<u8>) -> Option<bool> {
        if self.shape != (w, h) {
            self.clear();
            self.shape = (w, h);
            self.bytes = w as usize * h as usize * 4;
            for slot in &mut self.slots {
                (self.gen)(1, &mut slot.buffer);
                (self.bind)(PACK, slot.buffer);
                (self.data)(PACK, self.bytes as isize, std::ptr::null(), 0x88e1);
            }
        }
        let mut completed = None;
        // Choose the newest ready result; older completions cannot regress video.
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.fence == 0 {
                continue;
            }
            match (self.wait)(slot.fence as *mut c_void, 0, 0) {
                0x911a | 0x911c => {
                    if slot.sequence > self.delivered
                        && completed.is_none_or(|(_, seq)| slot.sequence > seq)
                    {
                        completed = Some((index, slot.sequence));
                    }
                }
                0x911b => {}
                _ => return None,
            }
        }
        if let Some((index, sequence)) = completed {
            let slot = &self.slots[index];
            (self.bind)(PACK, slot.buffer);
            let mapped = (self.map)(PACK, 0, self.bytes as isize, 1);
            if mapped.is_null() {
                return None;
            }
            output.resize(self.bytes, 0);
            output.copy_from_slice(std::slice::from_raw_parts(mapped as *const u8, self.bytes));
            if (self.unmap)(PACK) == 0 {
                return None;
            }
            self.delivered = sequence;
        }
        for slot in &mut self.slots {
            if slot.fence != 0 && slot.sequence <= self.delivered {
                (self.delete_sync)(slot.fence as *mut c_void);
                slot.fence = 0;
            }
        }
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.fence == 0) {
            self.sequence += 1;
            slot.sequence = self.sequence;
            (self.bind)(PACK, slot.buffer);
            (self.read)(
                0,
                0,
                w as i32,
                h as i32,
                0x1908,
                0x1401,
                std::ptr::null_mut(),
            );
            slot.fence = (self.fence)(0x9117, 0) as usize;
            if slot.fence == 0 {
                return None;
            }
            (self.flush)();
        }
        Some(completed.is_some())
    }

    unsafe fn clear(&mut self) {
        for slot in &mut self.slots {
            if slot.fence != 0 {
                (self.delete_sync)(slot.fence as *mut c_void);
            }
            if slot.buffer != 0 {
                (self.delete)(1, &slot.buffer);
            }
            *slot = Slot {
                buffer: 0,
                fence: 0,
                sequence: 0,
            };
        }
        self.sequence = 0;
        self.delivered = 0;
    }
}

impl Drop for Readback {
    fn drop(&mut self) {
        unsafe {
            self.clear();
        }
    }
}
