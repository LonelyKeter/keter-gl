use crate::{types::*, utility::*, GLFlag};

use gl::{self};
use std::{ffi::{CString}, marker::PhantomData};

pub type ShaderResult<T> = Result<T, ShaderError>;

pub struct Shader<K: ShaderKind> {
    id: GLuint,
    _kind: PhantomData<K>
}

impl<K: ShaderKind> Shader<K> {
    fn new(id: GLuint) -> Self {
        Self {
            id,
            _kind: PhantomData
        }
    }

    fn create() -> ShaderResult<Self> {
        let id = unsafe { gl::CreateShader(K::GL_VALUE) };

        if id != 0 {
            Ok(Self::new(id))
        } else {
            Err(ShaderError::CreationError)
        }
    }

    pub fn compile(&self, src: &ShaderSource) -> ShaderResult<()> {
        unsafe {
            gl::ShaderSource(self.id, 1, &src.string.as_ptr(), std::ptr::null());
            gl::CompileShader(self.id);
        }

        if self.get_compile_status() == gl::TRUE {
            Ok(())
        } else {
            let log = self.get_info_log();
            Err(ShaderError::CompilationError(log))
        }
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline(always)]
    fn get_parameter(&self, param: ShaderParameter) -> GLint {
        let mut val = 0;
        unsafe {
            gl::GetShaderiv(self.id, param.0, &mut val);
        }

        val
    }

    fn get_compile_status(&self) -> GLboolean {
        self.get_parameter(ShaderParameter::CompileStatus) as GLboolean
    }

    fn get_info_log_len(&self) -> GLint {
        self.get_parameter(ShaderParameter::InfoLogLength)
    }

    fn get_info_log(&self) -> CString {
        let log_len = self.get_info_log_len();
        let log = create_blank_cstring(log_len as usize + 1);

        unsafe {
            gl::GetShaderInfoLog(
                self.id,
                log_len,
                std::ptr::null_mut(),
                log.as_ptr() as *mut _,
            );
        }

        log
    }
}

impl<K: ShaderKind> Drop for Shader<K> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderBuilder<'a> {
    source: &'a ShaderSource,
}

impl<'a> ShaderBuilder<'a> {
    pub fn from_source(src: &'a ShaderSource) -> Self {
        Self { source: src }
    }

    pub fn finish<K: ShaderKind>(&self) -> ShaderResult<Shader<K>> {
        let shader = Shader::create()?;
        shader.compile(self.source)?;

        Ok(shader)
    }
}

pub trait ShaderKind: GLFlag {}

macro_rules! declare_shader_kind {
    ($kind:ident, $gl_value:expr) => {
        pub struct $kind;
        impl GLFlag for $kind {
            const GL_VALUE: GLenum = $gl_value;
        }
        impl ShaderKind for $kind {}
    };
}

declare_shader_kind!(Vertex, gl::VERTEX_SHADER);
declare_shader_kind!(TessEvaluation, gl::TESS_EVALUATION_SHADER);
declare_shader_kind!(TessControl, gl::TESS_CONTROL_SHADER);
declare_shader_kind!(Fragment, gl::FRAGMENT_SHADER);
declare_shader_kind!(Geometry, gl::GEOMETRY_SHADER);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct ShaderParameter(GLenum);

#[allow(dead_code, non_upper_case_globals)]
impl ShaderParameter {
    const Type: Self = Self(gl::SHADER_TYPE);
    const DeleteStatus: Self = Self(gl::DELETE_STATUS);
    const CompileStatus: Self = Self(gl::COMPILE_STATUS);
    const InfoLogLength: Self = Self(gl::INFO_LOG_LENGTH);
    const SourceLength: Self = Self(gl::SHADER_SOURCE_LENGTH);
}

pub enum ShaderError {
    CreationError,
    CompilationError(CString),
}

pub struct ShaderSource {
    string: CString,
}
