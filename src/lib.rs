#![feature(generic_const_exprs, negative_impls, adt_const_params)]

extern crate gl;

pub mod buffers;
pub mod color;
pub mod drawer;
pub mod errors;
pub mod layout;
pub mod primitives;
pub mod programs;
pub mod shaders;
pub mod types;
pub mod utility;
pub mod vao;

use crate::{buffers::*, types::*};
use std::ops::BitOr;

pub unsafe fn init<F>(load_fn: F) -> KeterGL
where
    F: FnMut(&'static str) -> *const std::ffi::c_void,
{
    KeterGL::init(load_fn)
}

pub struct KeterGL {}

impl KeterGL {
    fn init<F>(load_fn: F) -> Self
    where
        F: FnMut(&'static str) -> *const std::ffi::c_void,
    {
        gl::load_with(load_fn);

        Self {}
    }

    pub fn buffers(&self) -> buffers::Buffers {
        buffers::Buffers::new()
    }

    pub fn set_clear_color(&self, color: color::RGBA) {
        unsafe {
            gl::ClearColor(color.red, color.green, color.green, color.alpha);
        }
    }

    pub fn set_clear_depth(&self, depth: GLdouble) {
        unsafe {
            gl::ClearDepth(depth);
        }
    }

    pub fn clear(&self, buffers: BufferFlag) {
        unsafe {
            gl::Clear(buffers.0);
        }
    }

    pub fn flush(&self) {
        unsafe {
            gl::Flush();
        }
    }

    pub fn finish(&self) {
        unsafe {
            gl::Finish();
        }
    }

    pub fn use_buffer<Kind: BufferKind>(buff: Buffer<Kind>) {
        buff.bind();
    }
}

#[derive(Copy, Clone)]
pub struct BufferFlag(GLenum);

#[allow(dead_code)]
impl BufferFlag {
    pub const COLOR: Self = BufferFlag(gl::COLOR_BUFFER_BIT);
    pub const DEPTH: Self = BufferFlag(gl::DEPTH_BUFFER_BIT);
    pub const STENCIL: Self = BufferFlag(gl::COLOR_BUFFER_BIT);
}

impl BitOr for BufferFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub trait GLFlag {
    const GL_VALUE: GLenum;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
