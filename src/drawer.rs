use crate::{types::*, color};
use std::ops::{BitOr};


pub struct Drawer {

}

impl Drawer {
    pub fn set_clear_color(color: color::RGBA) {
        unsafe {
            gl::ClearColor(color.red, color.green, color.green, color.alpha);
        }
    }

    pub fn set_clear_depth(depth: GLdouble) {
        unsafe {
            gl::ClearDepth(depth);
        }
    }

    pub fn clear(buffers: BufferFlag) {
        unsafe {
            gl::Clear(buffers.0);
        }
    }

    pub fn set_color(color: color::RGB) {
        unsafe {
            gl::Color3f();
        }
    }
}

#[derive(Copy, Clone)]
pub struct BufferFlag(GLenum);

impl BufferFlag {
    const COLOR: Self = BufferFlag(gl::COLOR_BUFFER_BIT);
    const DEPTH: Self = BufferFlag(gl::DEPTH_BUFFER_BIT);
    //const ACCUMULATION: Self = BufferFlag(gl::ACCUM_BUFFER_BIT);
    const STENCIL: Self = BufferFlag(gl::COLOR_BUFFER_BIT);
}

impl BitOr for BufferFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0) 
    }
}