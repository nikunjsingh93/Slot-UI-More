use crate::lcd3x::mask_texture_rgba8;
use crate::power::{screen_brightness, screen_rect};
use crate::quad::Quad;
use crate::shaders::{GAME_FRAG, RECT_VERT};
use crate::surface::{GfxError, OUT_H, OUT_W};

pub const SCALE: u32 = 3;
pub const SRC_W: u32 = OUT_W / SCALE;
pub const SRC_H: u32 = OUT_H / SCALE;

pub const WHOLE_TEXTURE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct GamePass {
    prog: gl::types::GLuint,
    game: gl::types::GLuint,
    mask: gl::types::GLuint,
    u_rect: gl::types::GLint,
    u_bright: gl::types::GLint,
    u_uv: gl::types::GLint,
    power: f32,
    src: [f32; 4],
    dims: (u32, u32),
}

impl GamePass {
    pub fn new() -> Result<Self, GfxError> {
        let prog = crate::shaders::program(RECT_VERT, GAME_FRAG)?;
        let game = crate::gl::texture(SRC_W, SRC_H, gl::NEAREST, gl::CLAMP_TO_EDGE, gl::BGRA, None);
        let mask = crate::gl::texture(
            3,
            3,
            gl::NEAREST,
            gl::REPEAT,
            gl::RGBA,
            Some(&mask_texture_rgba8()),
        );
        let (u_rect, u_bright, u_uv);
        unsafe {
            gl::UseProgram(prog);
            gl::Uniform1i(crate::gl::uniform_location(prog, "u_game"), 0);
            gl::Uniform1i(crate::gl::uniform_location(prog, "u_mask"), 1);
            gl::Uniform2f(
                crate::gl::uniform_location(prog, "u_src"),
                SRC_W as f32,
                SRC_H as f32,
            );
            gl::Uniform2f(
                crate::gl::uniform_location(prog, "u_target"),
                OUT_W as f32,
                OUT_H as f32,
            );
            u_rect = crate::gl::uniform_location(prog, "u_rect");
            u_bright = crate::gl::uniform_location(prog, "u_bright");
            u_uv = crate::gl::uniform_location(prog, "u_uv");
        }
        Ok(GamePass {
            prog,
            game,
            mask,
            u_rect,
            u_bright,
            u_uv,
            power: 1.0,
            src: WHOLE_TEXTURE,
            dims: (SRC_W, SRC_H),
        })
    }

    pub fn set_power(&mut self, t: f32) {
        self.power = t.clamp(0.0, 1.0);
    }

    pub fn set_source_rect(&mut self, rect: [f32; 4]) {
        self.src = rect;
    }

    pub fn upload(&mut self, xrgb8888: &[u8]) {
        if xrgb8888.len() < (SRC_W * SRC_H * 4) as usize {
            return;
        }
        let dims = if xrgb8888.len() == 640 * 480 * 4 {
            (640, 480)
        } else if xrgb8888.len() == 256 * 192 * 4 {
            (256, 192)
        } else if xrgb8888.len() == 480 * 270 * 4 {
            (480, 270)
        } else if xrgb8888.len() == 480 * 272 * 4 {
            (480, 272)
        } else if xrgb8888.len() == 320 * 240 * 4 {
            (320, 240)
        } else {
            (SRC_W, SRC_H)
        };
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.game);
            if dims != self.dims {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    crate::gl::internal_format(gl::BGRA),
                    dims.0 as i32,
                    dims.1 as i32,
                    0,
                    gl::BGRA,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null(),
                );
                self.dims = dims;
            }
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                dims.0 as i32,
                dims.1 as i32,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                xrgb8888.as_ptr() as *const std::ffi::c_void,
            );
        }
    }

    pub fn draw(&self, quad: &Quad) {
        self.draw_source(self.game, quad, self.src);
    }

    pub fn draw_still(&self, tex: gl::types::GLuint, quad: &Quad) {
        self.draw_source(tex, quad, WHOLE_TEXTURE);
    }

    fn draw_source(&self, tex: gl::types::GLuint, quad: &Quad, src: [f32; 4]) {
        let (x, mut y, w, mut h) = screen_rect(self.power);
        if self.dims.0 == 480 && matches!(self.dims.1, 270 | 272) {
            let fitted = h * (4.0 / 3.0) / (480.0 / self.dims.1 as f32);
            y += (h - fitted) * 0.5;
            h = fitted;
        }
        unsafe {
            gl::UseProgram(self.prog);
            gl::Uniform2f(
                crate::gl::uniform_location(self.prog, "u_src"),
                self.dims.0 as f32,
                self.dims.1 as f32,
            );
            gl::Uniform1f(
                crate::gl::uniform_location(self.prog, "u_lcd"),
                if self.dims == (SRC_W, SRC_H) {
                    1.0
                } else {
                    0.0
                },
            );
            gl::Uniform4f(self.u_rect, x, y, w, h);
            gl::Uniform4f(self.u_uv, src[0], src[1], src[2], src[3]);
            gl::Uniform1f(self.u_bright, screen_brightness(self.power));
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.mask);
            gl::ActiveTexture(gl::TEXTURE0);
        }
        quad.draw();
    }
}

impl Drop for GamePass {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.game);
            gl::DeleteTextures(1, &self.mask);
            gl::DeleteProgram(self.prog);
        }
    }
}
