use std::os::raw::*;

pub type GLfloat = gl::types::GLfloat;
pub type GLdouble = gl::types::GLdouble;
pub type GLenum = gl::types::GLenum;
pub type GLuint = c_uint;
pub type GLint = c_int;
pub type GLboolean = c_uchar;
pub type GLsizei = c_int;
pub type GLsizeiptr = isize;

fn a() {
  let a: gl::types::GLsizeiptr;

}