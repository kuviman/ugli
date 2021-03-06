use super::*;

pub const TRUE: Bool = 1;
pub const FALSE: Bool = 0;

pub use gl::{
    ACTIVE_ATTRIBUTES, ACTIVE_UNIFORMS, ALPHA, ALWAYS, ARRAY_BUFFER, BACK, BLEND, CLAMP_TO_EDGE,
    COLOR_ATTACHMENT0, COLOR_BUFFER_BIT, COMPILE_STATUS, CULL_FACE, DEPTH_ATTACHMENT,
    DEPTH_BUFFER_BIT, DEPTH_COMPONENT, DEPTH_TEST, DYNAMIC_DRAW, FLOAT, FRAGMENT_SHADER,
    FRAMEBUFFER, FRAMEBUFFER_COMPLETE, FRONT, GREATER, INVALID_ENUM, INVALID_FRAMEBUFFER_OPERATION,
    INVALID_OPERATION, INVALID_VALUE, LEQUAL, LESS, LINEAR, LINEAR_MIPMAP_LINEAR, LINES, LINE_LOOP,
    LINE_STRIP, LINK_STATUS, NEAREST, NO_ERROR, ONE_MINUS_SRC_ALPHA, OUT_OF_MEMORY, POINTS,
    PROGRAM_POINT_SIZE, RENDERBUFFER, REPEAT, RGBA, SRC_ALPHA, STATIC_DRAW, TEXTURE0, TEXTURE_2D,
    TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, TRIANGLES,
    TRIANGLE_FAN, TRIANGLE_STRIP, UNPACK_ALIGNMENT, UNSIGNED_BYTE, VERTEX_SHADER,
};

#[cfg(target_os = "windows")]
pub const POINT_SPRITE: Enum = 0x8861;
