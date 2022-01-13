use crate::{
  KeterGL,
  types::*, 
  layout::*, GLFlag};
use std::marker::PhantomData;

pub type VertexBuffer<'a> = Buffer<'a, Array>;

pub struct Buffers<'a> {
  lifetime: std::marker::PhantomData<&'a KeterGL>
}

impl<'a> !Send for Buffers<'a> {}

impl<'a> Buffers<'a> {
  pub fn new() -> Self {
    Self {
      lifetime: PhantomData
    }
  }

  
  pub fn create_buffer<T: LayedOut, Kind: BufferKind>(&self, data: &[T], mode: BufferUsageMode) -> Buffer<Kind> {
    let mut buffer = Buffer::<Kind>::create();
    buffer.load_data(data, mode);

    buffer
  }  
}

pub struct Buffer<'a, Kind: BufferKind> {
  id: GLuint,
  _lifetime: PhantomData<&'a KeterGL>,
  _kind: PhantomData<Kind>
}

impl<'a, Kind: BufferKind> Buffer<'a, Kind> {
    fn new(id: GLuint) -> Self {
        Self {
            id, 
            _lifetime: PhantomData,
            _kind: PhantomData
        }
    }

  pub(crate) fn create() -> Self {
    let mut id = 0;
    unsafe {
      gl::GenBuffers(1, &mut id);
    }

    Self::new(id)
  }

  pub(crate) fn bind(&self) {
    unsafe {
      gl::BindBuffer(Kind::GL_VALUE, self.id);
    }
  }

  pub(crate) fn unbind() {
    unsafe {
      gl::BindBuffer(Kind::GL_VALUE, 0);
    }
  }

  pub fn load_data<T: LayedOut>(&mut self, data: &[T], mode: BufferUsageMode) {
    let size = std::mem::size_of::<T>() * data.len();

    unsafe {
      gl::NamedBufferData(
        self.id,
        size as GLsizeiptr, 
        data.as_ptr() as *const _, 
        mode.0);
    }
  }
}

impl<'a, Kind: BufferKind> Drop for Buffer<'a, Kind> {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &self.id);
    }
  }
}

pub trait BufferKind: GLFlag {}

macro_rules! declare_buffer_kind {
    ($kind:ident, $gl_value:expr) => {
        pub struct $kind;
        impl GLFlag for $kind {
            const GL_VALUE: GLenum = $gl_value;
        }
        impl BufferKind for $kind {}
    };
}

declare_buffer_kind!(Array, gl::ARRAY_BUFFER);
declare_buffer_kind!(ElementArray, gl::ELEMENT_ARRAY_BUFFER);
declare_buffer_kind!(PixelPack, gl::PIXEL_PACK_BUFFER);
declare_buffer_kind!(PixelUnpack, gl::PIXEL_UNPACK_BUFFER);
declare_buffer_kind!(CopyRead, gl::COPY_READ_BUFFER);
declare_buffer_kind!(CopyWrite, gl::COPY_WRITE_BUFFER);
declare_buffer_kind!(TransformFeedback, gl::TRANSFORM_FEEDBACK_BUFFER);
declare_buffer_kind!(Uniform, gl::UNIFORM_BUFFER);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct BufferUsageMode(GLenum);

#[allow(dead_code, non_upper_case_globals)]
impl BufferUsageMode {
  pub const StreamDraw: Self = Self(gl::STREAM_DRAW);
  pub const StreamRead: Self = Self(gl::STREAM_READ);
  pub const StreamCopy: Self = Self(gl::STREAM_COPY);
  pub const StaticDraw: Self = Self(gl::STATIC_DRAW);
  pub const StaticRead: Self = Self(gl::STATIC_READ);
  pub const StaticCopy: Self = Self(gl::STATIC_COPY);
  pub const DynamicDraw: Self = Self(gl::DYNAMIC_DRAW);
  pub const DynamicRead: Self = Self(gl::DYNAMIC_READ);
  pub const DynamicCopy: Self = Self(gl::DYNAMIC_COPY);
}