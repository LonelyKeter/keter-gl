use crate::{primitives::PrimitiveTypeData, types::*};

pub struct VertexAttribData {
    primitive_data: PrimitiveTypeData,
    offset: usize
}

pub trait LayedOut: std::marker::Sized {
    const ATTRIB_COUNT: usize;
    fn layout() -> Layout<{Self::ATTRIB_COUNT}>;
}

pub struct Layout<const ATTRIB_COUNT: usize> {
    stride: GLsizei,
    attrib_data: [VertexAttribData; ATTRIB_COUNT],
}

impl<const ATTRIB_COUNT: usize> Layout<ATTRIB_COUNT> {
    pub(crate) unsafe fn apply(&self) {
        for (index, attrib) in self.attrib_data.iter().enumerate() {
            gl::VertexAttribPointer(
                index as GLuint,
                attrib.primitive_data.component_count as GLint,
                attrib.primitive_data.component_type.value(),
                attrib.primitive_data.normalized as GLboolean,
                self.stride,
                attrib.offset as *const gl::types::GLvoid
            )
        }
    }
}
