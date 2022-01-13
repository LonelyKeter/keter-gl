use crate::{shaders::*, types::*};
use std::ffi::CString;

pub type ProgramResult<T> = Result<T, ProgramError>;

pub struct Program {
  id: GLuint
}

impl Program {
  fn create() -> ProgramResult<Self> {
    let id = unsafe { gl::CreateProgram() };

    if id != 0 {
      Ok(
        Self {
          id: id
        })
    } else {
      Err(ProgramError::CreationError)
    }
  }

  pub fn attach_shader<K: ShaderKind>(&mut self, shader: &Shader<K>) {
    unsafe {
      gl::AttachShader(self.id, shader.id());
    }
  }

  pub fn detach_shader<K: ShaderKind>(&mut self, shader: &Shader<K>) {
    unsafe {
      gl::DetachShader(self.id, shader.id());
    }
  }

  pub fn link(&mut self) -> ProgramResult<()> {
    unsafe {
      gl::LinkProgram(self.id);
    }

    let status = self.get_link_status();

    if status == gl::TRUE {
      Ok(())
    } else {
      Err(ProgramError::LinkError(self.get_info_log()))
    }
  }

  unsafe fn get_parameter(&self, param: ProgramParameter, value: *mut GLint) {
    gl::GetProgramiv(self.id, param.0, value);
  }

  fn get_info_log(&self) -> CString {
    use crate::utility::create_blank_cstring;

    let log_len = self.get_info_log_len();
    let log = create_blank_cstring(log_len as usize + 1);

    unsafe {
      gl::GetProgramInfoLog(
        self.id, 
        log_len, 
        std::ptr::null_mut(), 
        log.as_ptr() as *mut _);
    }

    log
  }

  fn get_info_log_len(&self) -> GLint {
    let mut len = 0;
    unsafe {
      self.get_parameter(ProgramParameter::InfoLogLength, &mut len);
    }

    len
  }

  fn get_link_status(&self) -> GLboolean {
    let mut status = 1;
    unsafe {
      self.get_parameter(ProgramParameter::LinkStatus, &mut status);
    }

    status as GLboolean
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct ProgramParameter(GLenum);

#[allow(dead_code, non_upper_case_globals)]
impl ProgramParameter {
    const DeleteStatus: Self = Self(gl::DELETE_STATUS);
    const LinkStatus: Self = Self(gl::LINK_STATUS);
    const ValidateStatus: Self = Self(gl::VALIDATE_STATUS);
    const InfoLogLength: Self = Self(gl::INFO_LOG_LENGTH);
    const AttachedShaders: Self = Self(gl::ATTACHED_SHADERS);
    const ActiveAtomicCounterBuffers: Self = Self(gl::ACTIVE_ATOMIC_COUNTER_BUFFERS);
    const ActiveAttributes: Self = Self(gl::ACTIVE_ATTRIBUTES);
    const ActiveAttributeMaxLength: Self = Self(gl::ACTIVE_ATTRIBUTE_MAX_LENGTH);
    const ActiveUniforms: Self = Self(gl::ACTIVE_UNIFORMS);
    const ActiveUniformMaxLength: Self = Self(gl::ACTIVE_UNIFORM_MAX_LENGTH);
    const ProgramBinaryLength: Self = Self(gl::PROGRAM_BINARY_LENGTH);
    const ComputeWorkGroupSize: Self = Self(gl::COMPUTE_WORK_GROUP_SIZE);
    const TransformFeedbackBufferMode: Self = Self(gl::TRANSFORM_FEEDBACK_BUFFER_MODE);
    const TransformFeedbackVaryings: Self = Self(gl::TRANSFORM_FEEDBACK_VARYINGS);
    const TransformFeedbackVaryingMaxLength: Self = Self(gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH);
    const GeometryVerticesOut: Self = Self(gl::GEOMETRY_VERTICES_OUT);
    const GeometryInputType: Self = Self(gl::GEOMETRY_INPUT_TYPE);
    const GeometryOutputType: Self = Self(gl::GEOMETRY_OUTPUT_TYPE);
}

pub enum ProgramError {
  CreationError,
  ShaderAllreadyAttached,
  LinkError(CString)
}