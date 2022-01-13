use crate::types::*;


#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Error(GLenum);

#[allow(dead_code, non_upper_case_globals)]
impl Error {
  const NoError: Self = Self(gl::NO_ERROR);
  const InvalidEnum: Self = Self(gl::INVALID_ENUM);
  const InvalidValue: Self = Self(gl::INVALID_VALUE);
  const InvalidOperation: Self = Self(gl::INVALID_OPERATION);
  const InvalidFramebufferOperation: Self = Self(gl::INVALID_FRAMEBUFFER_OPERATION);
  const OutOfMemory: Self = Self(gl::OUT_OF_MEMORY);
  const StackOverflow: Self = Self(gl::STACK_OVERFLOW);
  const StackUnderflow: Self = Self(gl::STACK_UNDERFLOW);

  pub fn get() -> Option<Self> {
    unsafe {
      let error = Error(gl::GetError());

      match error {
        Error::NoError => None,
        err => Some(err)
      }
    }
  }

  pub fn get_all(buff: &mut Vec<Error>) {
    while let Some(error) = Self::get() {
      buff.push(error);
    }
  }
}