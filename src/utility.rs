use crate::types::*;

use std::ffi::{CString};

pub fn create_blank_cstring(len: usize) -> CString {
  let mut buff = vec![b' '; len as usize + 1];
  buff[len as usize] = b'\0';

  unsafe {
    CString::from_vec_unchecked(buff)
  }
}