use crate::types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RGBA {
    pub red: GLfloat,
    pub green: GLfloat,
    pub blue: GLfloat,
    pub alpha: GLfloat,
}

impl RGBA {
    fn new(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        }
    }
}

type RGBAtuple = (GLfloat, GLfloat, GLfloat, GLfloat);

impl From<RGBAtuple> for RGBA {
    fn from(value: RGBAtuple) -> Self {
        Self {
            red: value.0,
            green: value.1,
            blue: value.2,
            alpha: value.3
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RGB {
    pub red: GLfloat,
    pub green: GLfloat,
    pub blue: GLfloat,
}

impl RGB {
    fn new(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
        }
    }
}

type RGBtuple = (GLfloat, GLfloat, GLfloat);

impl From<RGBtuple> for RGB {
    fn from(value: RGBtuple) -> Self {
        Self {
            red: value.0,
            green: value.1,
            blue: value.2,
        }
    }
}