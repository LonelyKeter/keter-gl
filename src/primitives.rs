use crate::{
  layout::{LayedOut, Layout},
  types::*
};

pub unsafe trait Primitive {
  fn type_data() -> PrimitiveTypeData;
}

pub struct PrimitiveTypeData {
  pub component_type: ComponentType,
  pub component_count: u8,
  pub normalized: bool
}

impl PrimitiveTypeData {
  #[inline(always)]
  fn component_type(&self) -> ComponentType { self.component_type }
  #[inline(always)]
  fn component_count(&self) -> u8 { self.component_count }
  #[inline(always)]
  fn normalized(&self) -> bool { self.normalized }
  #[inline(always)]
  const fn size(&self) -> GLsizei { 
    self.component_type.size() * self.component_count as GLsizei
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ComponentType(GLenum);

impl ComponentType {
  pub const I8: Self = Self(gl::BYTE);
  pub const U8: Self = Self(gl::UNSIGNED_BYTE);
  pub const I16: Self = Self(gl::SHORT);
  pub const U16: Self = Self(gl::UNSIGNED_SHORT);
  pub const I32: Self = Self(gl::INT);
  pub const U32: Self = Self(gl::UNSIGNED_INT);

  pub const F16: Self = Self(gl::HALF_FLOAT);
  pub const F32: Self = Self(gl::FLOAT);

  pub const fn size(self) -> GLsizei {
    match self {
      Self::I8 => 1,       
      Self::U8 => 1,       
      Self::I16 => 2,       
      Self::U16 => 2,
      Self::I32 => 4,
      Self::U32 => 4,

      Self::F16 => 2,
      Self::F32 => 4,
      _ => panic!("Tried to get size of incorrectly initialized ComponentType value") 
    }
  }

  pub const fn value(self) -> GLenum {
      self.0
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GLi8(i8);

impl GLi8 {
  pub fn new(val: i8) -> Self {
    Self(val)
  }
}

impl From<i8> for GLi8 {
  fn from(val: i8) -> Self {
    Self::new(val)
  }
}


pub struct GLu8(u8);

impl GLu8 {
  pub fn new(val: u8) -> Self {
    Self(val)
  }
}

impl From<u8> for GLu8 {
  fn from(val: u8) -> Self {
    Self::new(val)
  }
}
