use gl::{self, types::*};
use std::ffi;

pub struct Shader {
    id: GLuint
}

impl Shader {
    pub fn from_cstr(src: &str) -> Self {
        let id = unsafe {gl::CreateShader}
    }
}