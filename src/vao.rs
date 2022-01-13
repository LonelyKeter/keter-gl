use crate::{
  *,
  types::*,
  buffers::*
};

pub type VaoResult<T> = Result<T, VaoError>;

pub struct VertexArrayObject<'a> {
  id: GLuint,
  vertex_buffer: VertexBuffer<'a>
}

impl<'a> VertexArrayObject<'a> {
  pub(crate) fn create(buffer: VertexBuffer<'a>) -> Self {
    let mut id = 0;     
    unsafe {
      gl::GenVertexArrays(1, &mut id);
    }

    VertexArrayObject {
      id: id,
      vertex_buffer: buffer
    }
  }

  pub(crate) fn bind(&self) {
    unsafe {
      gl::BindVertexArray(self.id);
      self.vertex_buffer.bind();
    }
  }

  pub(crate) fn unbind() {
    unsafe {
      gl::BindVertexArray(0);
    }
  }
}

impl<'a> Drop for VertexArrayObject<'a> {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteVertexArrays(1, &self.id)
    }
  }
}

pub enum VaoError {

}