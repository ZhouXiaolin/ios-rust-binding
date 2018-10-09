#![allow(unused_imports)]

use gles::consts::*;
use gles::es20::ffi::*;
use gles::types::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Blending {
    None,
    Default,
    Additive,
    Subtractive,
    Multiply,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CullFace {
    None,
    Back,
    Front,
    FrontAndBack,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Depth {
    None,
    Never,
    LessThan,
    Equal,
    LessThanOrEqual,
    GreaterThan,
    NotEqual,
    GreaterThanOrEqual,
    Always,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FilterMode {
    None,
    Linear,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DrawMode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    //    LineStripAdjacency,
    //    LineAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    //    TriangleStripAdjacency,
    //    TrianglesAdjacency,
    //    Patches,
}

impl From<GLenum> for DrawMode {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_POINTS => DrawMode::Points,
            GL_LINE_STRIP => DrawMode::LineStrip,
            GL_LINE_LOOP => DrawMode::LineLoop,
            GL_LINES => DrawMode::Lines,
            //            GL_LINE_STRIP_ADJACENCY => DrawMode::LineStripAdjacency,
            //            GL_LINES_ADJACENCY => DrawMode::LineAdjacency,
            GL_TRIANGLE_STRIP => DrawMode::TriangleStrip,
            GL_TRIANGLE_FAN => DrawMode::TriangleFan,
            GL_TRIANGLES => DrawMode::Triangles,
            //            GL_TRIANGLE_STRIP_ADJACENCY => DrawMode::TriangleStripAdjacency,
            //            GL_TRIANGLES_ADJACENCY => DrawMode::TrianglesAdjacency,
            //            GL_PATCHES => DrawMode::Patches,
            _ => panic!("Invalid GLenum {:?} for DrawMode", gl_enum),
        }
    }
}

impl<'a> From<&'a DrawMode> for GLenum {
    #[inline]
    fn from(draw_mode: &'a DrawMode) -> Self {
        match draw_mode {
            &DrawMode::Points => GL_POINTS,
            &DrawMode::LineStrip => GL_LINE_STRIP,
            &DrawMode::LineLoop => GL_LINE_LOOP,
            &DrawMode::Lines => GL_LINES,
            //            &DrawMode::LineStripAdjacency => GL_LINE_STRIP_ADJACENCY,
            //            &DrawMode::LineAdjacency => GL_LINES_ADJACENCY,
            &DrawMode::TriangleStrip => GL_TRIANGLE_STRIP,
            &DrawMode::TriangleFan => GL_TRIANGLE_FAN,
            &DrawMode::Triangles => GL_TRIANGLES,
            //            &DrawMode::TriangleStripAdjacency => GL_TRIANGLE_STRIP_ADJACENCY,
            //            &DrawMode::TrianglesAdjacency => GL_TRIANGLES_ADJACENCY,
            //            &DrawMode::Patches => GL_PATCHES,
        }
    }
}

impl From<DrawMode> for GLenum {
    #[inline(always)]
    fn from(draw_mode: DrawMode) -> Self {
        From::from(&draw_mode)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IndexKind {
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
}

impl From<GLenum> for IndexKind {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_UNSIGNED_BYTE => IndexKind::UnsignedByte,
            GL_UNSIGNED_SHORT => IndexKind::UnsignedShort,
            GL_UNSIGNED_INT => IndexKind::UnsignedInt,
            _ => panic!("Invalid GLenum {:?} for IndexKind", gl_enum),
        }
    }
}
impl<'a> From<&'a IndexKind> for GLenum {
    #[inline]
    fn from(index_kind: &'a IndexKind) -> Self {
        match index_kind {
            &IndexKind::UnsignedByte => GL_UNSIGNED_BYTE,
            &IndexKind::UnsignedShort => GL_UNSIGNED_SHORT,
            &IndexKind::UnsignedInt => GL_UNSIGNED_INT,
        }
    }
}
impl From<IndexKind> for GLenum {
    #[inline(always)]
    fn from(index_kind: IndexKind) -> Self {
        From::from(&index_kind)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    //    StackUnderflow,
    //    StackOverflow,
    OutOfMemory,
}

impl From<GLenum> for Error {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_NO_ERROR => Error::NoError,
            GL_INVALID_ENUM => Error::InvalidEnum,
            GL_INVALID_VALUE => Error::InvalidValue,
            GL_INVALID_OPERATION => Error::InvalidOperation,
            GL_INVALID_FRAMEBUFFER_OPERATION => Error::InvalidFramebufferOperation,
            //            GL_STACK_UNDERFLOW => Error::StackUnderflow,
            //            GL_STACK_OVERFLOW => Error::StackOverflow,
            GL_OUT_OF_MEMORY => Error::OutOfMemory,
            _ => panic!("Invalid GLenum {:?} for Error", gl_enum),
        }
    }
}
impl<'a> From<&'a Error> for GLenum {
    #[inline]
    fn from(error: &'a Error) -> Self {
        match error {
            &Error::NoError => GL_NO_ERROR,
            &Error::InvalidEnum => GL_INVALID_ENUM,
            &Error::InvalidValue => GL_INVALID_VALUE,
            &Error::InvalidOperation => GL_INVALID_OPERATION,
            &Error::InvalidFramebufferOperation => GL_INVALID_FRAMEBUFFER_OPERATION,
            //            &Error::StackUnderflow => GL_STACK_UNDERFLOW,
            //            &Error::StackOverflow => GL_STACK_OVERFLOW,
            &Error::OutOfMemory => GL_OUT_OF_MEMORY,
        }
    }
}
impl From<Error> for GLenum {
    #[inline(always)]
    fn from(error: Error) -> Self {
        From::from(&error)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Usage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

impl From<GLenum> for Usage {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_STREAM_DRAW => Usage::StreamDraw,
            GL_STREAM_READ => Usage::StreamRead,
            GL_STREAM_COPY => Usage::StreamCopy,

            GL_STATIC_DRAW => Usage::StaticDraw,
            GL_STATIC_READ => Usage::StaticRead,
            GL_STATIC_COPY => Usage::StaticCopy,

            GL_DYNAMIC_DRAW => Usage::DynamicDraw,
            GL_DYNAMIC_READ => Usage::DynamicRead,
            GL_DYNAMIC_COPY => Usage::DynamicCopy,

            _ => panic!("Invalid GLenum {:?} for Usage", gl_enum),
        }
    }
}
impl<'a> From<&'a Usage> for GLenum {
    #[inline]
    fn from(usage: &'a Usage) -> Self {
        match usage {
            &Usage::StreamDraw => GL_STREAM_DRAW,
            &Usage::StreamRead => GL_STREAM_READ,
            &Usage::StreamCopy => GL_STREAM_COPY,

            &Usage::StaticDraw => GL_STATIC_DRAW,
            &Usage::StaticRead => GL_STATIC_READ,
            &Usage::StaticCopy => GL_STATIC_COPY,

            &Usage::DynamicDraw => GL_DYNAMIC_DRAW,
            &Usage::DynamicRead => GL_DYNAMIC_READ,
            &Usage::DynamicCopy => GL_DYNAMIC_COPY,
        }
    }
}
impl From<Usage> for GLenum {
    #[inline(always)]
    fn from(usage: Usage) -> Self {
        From::from(&usage)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BufferTarget {
    Array,
    //    AtomicCounter,
    //    CopyRead,
    //    CopyWrite,
    //    DispatchIndirect,
    //    DrawIndirect,
    ElementArray,
    PixelPack,
    PixelUnpack,
    //    Query,
    //    ShaderStorage,
    //    Texture,
    TransformFeedback,
    //    UniformKind,
}

impl From<GLenum> for BufferTarget {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_ARRAY_BUFFER => BufferTarget::Array,
            //            GL_ATOMIC_COUNTER_BUFFER => BufferTarget::AtomicCounter,
            //            GL_COPY_READ_BUFFER => BufferTarget::CopyRead,
            //            GL_COPY_WRITE_BUFFER => BufferTarget::CopyWrite,
            //            GL_DISPATCH_INDIRECT_BUFFER => BufferTarget::DispatchIndirect,
            //            GL_DRAW_INDIRECT_BUFFER => BufferTarget::DrawIndirect,
            GL_ELEMENT_ARRAY_BUFFER => BufferTarget::ElementArray,
            GL_PIXEL_PACK_BUFFER => BufferTarget::PixelPack,
            GL_PIXEL_UNPACK_BUFFER => BufferTarget::PixelUnpack,
            //            GL_QUERY_BUFFER => BufferTarget::Query,
            //            GL_SHADER_STORAGE_BUFFER => BufferTarget::ShaderStorage,
            //            GL_TEXTURE_BUFFER => BufferTarget::Texture,
            GL_TRANSFORM_FEEDBACK_BUFFER => BufferTarget::TransformFeedback,
            //            GL_UNIFORM_BUFFER => BufferTarget::UniformKind,
            _ => panic!("Invalid GLenum {:?} for BufferTarget", gl_enum),
        }
    }
}
impl<'a> From<&'a BufferTarget> for GLenum {
    #[inline]
    fn from(buffer_target: &'a BufferTarget) -> Self {
        match buffer_target {
            &BufferTarget::Array => GL_ARRAY_BUFFER,
            //            &BufferTarget::AtomicCounter => GL_ATOMIC_COUNTER_BUFFER,
            //            &BufferTarget::CopyRead => GL_COPY_READ_BUFFER,
            //            &BufferTarget::CopyWrite => GL_COPY_WRITE_BUFFER,
            //            &BufferTarget::DispatchIndirect => GL_DISPATCH_INDIRECT_BUFFER,
            //            &BufferTarget::DrawIndirect => GL_DRAW_INDIRECT_BUFFER,
            &BufferTarget::ElementArray => GL_ELEMENT_ARRAY_BUFFER,
            &BufferTarget::PixelPack => GL_PIXEL_PACK_BUFFER,
            &BufferTarget::PixelUnpack => GL_PIXEL_UNPACK_BUFFER,
            //            &BufferTarget::Query => GL_QUERY_BUFFER,
            //            &BufferTarget::ShaderStorage => GL_SHADER_STORAGE_BUFFER,
            //            &BufferTarget::Texture => GL_TEXTURE_BUFFER,
            &BufferTarget::TransformFeedback => GL_TRANSFORM_FEEDBACK_BUFFER,
            //            &BufferTarget::UniformKind => GL_UNIFORM_BUFFER,
        }
    }
}
impl From<BufferTarget> for GLenum {
    #[inline(always)]
    fn from(buffer_target: BufferTarget) -> Self {
        From::from(&buffer_target)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureKind {
    //    Texture1D,
    Texture2D,
    Texture3D,
    //    Texture1DArray,
    Texture2DArray,
    //    TextureRectangle,
    TextureCubeMap,
    //    TextureCubeMapArray,
    //    TextureBuffer,
    //    Texture2DMultiSameple,
    //    Texture2DMultiSamepleArray,
}

impl From<GLenum> for TextureKind {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            //            GL_TEXTURE_1D => TextureKind::Texture1D,
            GL_TEXTURE_2D => TextureKind::Texture2D,
            GL_TEXTURE_3D => TextureKind::Texture3D,
            //            GL_TEXTURE_1D_ARRAY => TextureKind::Texture1DArray,
            GL_TEXTURE_2D_ARRAY => TextureKind::Texture2DArray,
            //            GL_TEXTURE_RECTANGLE => TextureKind::TextureRectangle,
            GL_TEXTURE_CUBE_MAP => TextureKind::TextureCubeMap,
            //            GL_TEXTURE_CUBE_MAP_ARRAY => TextureKind::TextureCubeMapArray,
            //            GL_TEXTURE_BUFFER => TextureKind::TextureBuffer,
            //            GL_TEXTURE_2D_MULTISAMPLE => TextureKind::Texture2DMultiSameple,
            //            GL_TEXTURE_2D_MULTISAMPLE_ARRAY => TextureKind::Texture2DMultiSamepleArray,
            _ => panic!("Invalid GLenum {:?} for TextureKind", gl_enum),
        }
    }
}
impl<'a> From<&'a TextureKind> for GLenum {
    #[inline]
    fn from(texture_kind: &'a TextureKind) -> Self {
        match texture_kind {
            //            &TextureKind::Texture1D => GL_TEXTURE_1D,
            &TextureKind::Texture2D => GL_TEXTURE_2D,
            &TextureKind::Texture3D => GL_TEXTURE_3D,
            //            &TextureKind::Texture1DArray => GL_TEXTURE_1D_ARRAY,
            &TextureKind::Texture2DArray => GL_TEXTURE_2D_ARRAY,
            //            &TextureKind::TextureRectangle => GL_TEXTURE_RECTANGLE,
            &TextureKind::TextureCubeMap => GL_TEXTURE_CUBE_MAP,
            //            &TextureKind::TextureCubeMapArray => GL_TEXTURE_CUBE_MAP_ARRAY,
            //            &TextureKind::TextureBuffer => GL_TEXTURE_BUFFER,
            //            &TextureKind::Texture2DMultiSameple => GL_TEXTURE_2D_MULTISAMPLE,
            //            &TextureKind::Texture2DMultiSamepleArray => GL_TEXTURE_2D_MULTISAMPLE_ARRAY,
        }
    }
}
impl From<TextureKind> for GLenum {
    #[inline(always)]
    fn from(texture_kind: TextureKind) -> Self {
        From::from(&texture_kind)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Attachment {
    Color(usize),
    Depth,
    Stencil,
}

impl From<GLenum> for Attachment {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_DEPTH_ATTACHMENT => Attachment::Depth,
            GL_STENCIL_ATTACHMENT => Attachment::Stencil,
            _ => if gl_enum >= GL_COLOR_ATTACHMENT0 {
                Attachment::Color((gl_enum - GL_COLOR_ATTACHMENT0) as usize)
            } else {
                panic!("Invalid GLenum {:?} for Attachment", gl_enum)
            },
        }
    }
}
impl<'a> From<&'a Attachment> for GLenum {
    #[inline]
    fn from(attachment: &'a Attachment) -> Self {
        match attachment {
            &Attachment::Color(index) => GL_COLOR_ATTACHMENT0 + (index as GLenum),
            &Attachment::Depth => GL_DEPTH_ATTACHMENT,
            &Attachment::Stencil => GL_STENCIL_ATTACHMENT,
        }
    }
}
impl From<Attachment> for GLenum {
    #[inline(always)]
    fn from(attachment: Attachment) -> Self {
        From::from(&attachment)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DataFormat {
    Red,
    RG,
    RGB,
    //    BGR,
    RGBA,
    //    BGRA,
    RedInteger,
    RGInteger,
    RGBInteger,
    //    BGRInteger,
    RGBAInteger,
    //    BGRAInteger,
    StencilInteger,
    DepthComponent,
    DepthStencil,
}

impl From<GLenum> for DataFormat {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_RED => DataFormat::Red,
            GL_RG => DataFormat::RG,
            GL_RGB => DataFormat::RGB,
            //            GL_BGR => DataFormat::BGR,
            GL_RGBA => DataFormat::RGBA,
            //            GL_BGRA => DataFormat::BGRA,
            GL_RED_INTEGER => DataFormat::RedInteger,
            GL_RG_INTEGER => DataFormat::RGInteger,
            GL_RGB_INTEGER => DataFormat::RGBInteger,
            //            GL_BGR_INTEGER => DataFormat::BGRInteger,
            GL_RGBA_INTEGER => DataFormat::RGBAInteger,
            //            GL_BGRA_INTEGER => DataFormat::BGRAInteger,
            GL_STENCIL_INDEX => DataFormat::StencilInteger,
            GL_DEPTH_COMPONENT => DataFormat::DepthComponent,
            GL_DEPTH_STENCIL => DataFormat::DepthStencil,
            _ => panic!("Invalid GLenum {:?} for DataFormat", gl_enum),
        }
    }
}
impl<'a> From<&'a DataFormat> for GLenum {
    #[inline]
    fn from(data_format: &'a DataFormat) -> Self {
        match data_format {
            &DataFormat::Red => GL_RED,
            &DataFormat::RG => GL_RG,
            &DataFormat::RGB => GL_RGB,
            //            &DataFormat::BGR => GL_BGR,
            &DataFormat::RGBA => GL_RGBA,
            //            &DataFormat::BGRA => GL_BGRA,
            &DataFormat::RedInteger => GL_RED_INTEGER,
            &DataFormat::RGInteger => GL_RG_INTEGER,
            &DataFormat::RGBInteger => GL_RGB_INTEGER,
            //            &DataFormat::BGRInteger => GL_BGR_INTEGER,
            &DataFormat::RGBAInteger => GL_RGBA_INTEGER,
            //            &DataFormat::BGRAInteger => GL_BGRA_INTEGER,
            &DataFormat::StencilInteger => GL_STENCIL_INDEX,
            &DataFormat::DepthComponent => GL_DEPTH_COMPONENT,
            &DataFormat::DepthStencil => GL_DEPTH_STENCIL,
        }
    }
}
impl From<DataFormat> for GLenum {
    #[inline(always)]
    fn from(data_format: DataFormat) -> Self {
        From::from(&data_format)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DataKind {
    UnsignedByte,
    Byte,
    UnsignedShort,
    Short,
    UnsignedInt,
    Int,
    Float,
    Bool,
    //    UnsignedByte332,
    //    UnsignedByte223Rev,
    //
    //    UnsignedShort565,
    //    UnsignedShort565Rev,
    //
    //    UnsignedShort4444,
    //    UnsignedShort4444Rev,
    //
    //    UnsignedShort5551,
    //    UnsignedShort1555Rev,
    //
    //    UnsignedInt8888,
    //    UnsignedInt8888Rev,
    //
    //    UnsignedInt1010102,
    //    UnsignedInt2101010Rev,
}

impl From<GLenum> for DataKind {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_UNSIGNED_BYTE => DataKind::UnsignedByte,
            GL_BYTE => DataKind::Byte,
            GL_UNSIGNED_SHORT => DataKind::UnsignedShort,
            GL_SHORT => DataKind::Short,
            GL_UNSIGNED_INT => DataKind::UnsignedInt,
            GL_INT => DataKind::Int,
            GL_FLOAT => DataKind::Float,
            GL_BOOL => DataKind::Bool,

            //            GL_UNSIGNED_BYTE_3_3_2 => DataKind::UnsignedByte332,
            //            GL_UNSIGNED_BYTE_2_3_3_REV => DataKind::UnsignedByte223Rev,
            //
            //            GL_UNSIGNED_SHORT_5_6_5 => DataKind::UnsignedShort565,
            //            GL_UNSIGNED_SHORT_5_6_5_REV => DataKind::UnsignedShort565Rev,
            //
            //            GL_UNSIGNED_SHORT_4_4_4_4 => DataKind::UnsignedShort4444,
            //            GL_UNSIGNED_SHORT_4_4_4_4_REV => DataKind::UnsignedShort4444Rev,
            //
            //            GL_UNSIGNED_SHORT_5_5_5_1 => DataKind::UnsignedShort5551,
            //            GL_UNSIGNED_SHORT_1_5_5_5_REV => DataKind::UnsignedShort1555Rev,
            //
            //            GL_UNSIGNED_INT_8_8_8_8 => DataKind::UnsignedInt8888,
            //            GL_UNSIGNED_INT_8_8_8_8_REV => DataKind::UnsignedInt8888Rev,
            //
            //            GL_UNSIGNED_INT_10_10_10_2 => DataKind::UnsignedInt1010102,
            //            GL_UNSIGNED_INT_2_10_10_10_REV => DataKind::UnsignedInt2101010Rev,
            _ => panic!("Invalid GLenum {:?} for DataKind", gl_enum),
        }
    }
}
impl<'a> From<&'a DataKind> for GLenum {
    #[inline]
    fn from(data_kind: &'a DataKind) -> Self {
        match data_kind {
            &DataKind::UnsignedByte => GL_UNSIGNED_BYTE,
            &DataKind::Byte => GL_BYTE,
            &DataKind::UnsignedShort => GL_UNSIGNED_SHORT,
            &DataKind::Short => GL_SHORT,
            &DataKind::UnsignedInt => GL_UNSIGNED_INT,
            &DataKind::Int => GL_INT,
            &DataKind::Float => GL_FLOAT,
            &DataKind::Bool => GL_BOOL,
            //            &DataKind::UnsignedByte332 => GL_UNSIGNED_BYTE_3_3_2,
//            &DataKind::UnsignedByte223Rev => GL_UNSIGNED_BYTE_2_3_3_REV,

//            &DataKind::UnsignedShort565 => GL_UNSIGNED_SHORT_5_6_5,
//            &DataKind::UnsignedShort565Rev => GL_UNSIGNED_SHORT_5_6_5_REV,
//
//            &DataKind::UnsignedShort4444 => GL_UNSIGNED_SHORT_4_4_4_4,
//            &DataKind::UnsignedShort4444Rev => GL_UNSIGNED_SHORT_4_4_4_4_REV,
//
//            &DataKind::UnsignedShort5551 => GL_UNSIGNED_SHORT_5_5_5_1,
//            &DataKind::UnsignedShort1555Rev => GL_UNSIGNED_SHORT_1_5_5_5_REV,
//
//            &DataKind::UnsignedInt8888 => GL_UNSIGNED_INT_8_8_8_8,
//            &DataKind::UnsignedInt8888Rev => GL_UNSIGNED_INT_8_8_8_8_REV,
//
//            &DataKind::UnsignedInt1010102 => GL_UNSIGNED_INT_10_10_10_2,
//            &DataKind::UnsignedInt2101010Rev => GL_UNSIGNED_INT_2_10_10_10_REV,
        }
    }
}
impl From<DataKind> for GLenum {
    #[inline(always)]
    fn from(data_kind: DataKind) -> Self {
        From::from(&data_kind)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InternalFormat {
    R8,
    R8_SNORM,
    R16F,
    R32F,
    R8UI,
    R8I,
    R16UI,
    R16I,
    R32UI,
    R32I,
    RG8,
    RG8_SNORM,
    RG16F,
    RG32F,
    RG8UI,
    RG8I,
    RG16UI,
    RG16I,
    RG32UI,
    RG32I,
    RGB8,
    SRGB8,
    RGB565,
    RGB8_SNORM,
    R11F_G11F_B10F,
    RGB9_E5,
    RGB16F,
    RGB32F,
    RGB8UI,
    RGB8I,
    RGB16UI,
    RGB16I,
    RGB32UI,
    RGB32I,
    RGBA,
    RGBA8,
    SRGB8_ALPHA8,
    RGBA8_SNORM,
    RGB5_A1,
    RGBA4,
    RGB10_A2,
    RGBA16F,
    RGBA32F,
    RGBA8UI,
    RGBA8I,
    RGB10_A2UI,
    RGBA16UI,
    RGBA16I,
    RGBA32I,
    RGBA32UI,
    DepthComponent16,
    DepthComponent24,
    DepthComponent32F,
    Depth24Stencil8,
    Depth32FStencil8,
    StencilIndex8,
}

impl From<GLenum> for InternalFormat {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_R8 => InternalFormat::R8,
            GL_R8_SNORM => InternalFormat::R8_SNORM,
            GL_R16F => InternalFormat::R16F,
            GL_R32F => InternalFormat::R32F,
            GL_R8UI => InternalFormat::R8UI,
            GL_R8I => InternalFormat::R8I,
            GL_R16UI => InternalFormat::R16UI,
            GL_R16I => InternalFormat::R16I,
            GL_R32UI => InternalFormat::R32UI,
            GL_R32I => InternalFormat::R32I,
            GL_RG8 => InternalFormat::RG8,
            GL_RG8_SNORM => InternalFormat::RG8_SNORM,
            GL_RG16F => InternalFormat::RG16F,
            GL_RG32F => InternalFormat::RG32F,
            GL_RG8UI => InternalFormat::RG8UI,
            GL_RG8I => InternalFormat::RG8I,
            GL_RG16UI => InternalFormat::RG16UI,
            GL_RG16I => InternalFormat::RG16I,
            GL_RG32UI => InternalFormat::RG32UI,
            GL_RG32I => InternalFormat::RG32I,
            GL_RGB8 => InternalFormat::RGB8,
            GL_SRGB8 => InternalFormat::SRGB8,
            GL_RGB565 => InternalFormat::RGB565,
            GL_RGB8_SNORM => InternalFormat::RGB8_SNORM,
            GL_R11F_G11F_B10F => InternalFormat::R11F_G11F_B10F,
            GL_RGB9_E5 => InternalFormat::RGB9_E5,
            GL_RGB16F => InternalFormat::RGB16F,
            GL_RGB32F => InternalFormat::RGB32F,
            GL_RGB8UI => InternalFormat::RGB8UI,
            GL_RGB8I => InternalFormat::RGB8I,
            GL_RGB16UI => InternalFormat::RGB16UI,
            GL_RGB16I => InternalFormat::RGB16I,
            GL_RGB32UI => InternalFormat::RGB32UI,
            GL_RGB32I => InternalFormat::RGB32I,
            GL_RGBA => InternalFormat::RGBA,
            GL_RGBA8 => InternalFormat::RGBA8,
            GL_SRGB8_ALPHA8 => InternalFormat::SRGB8_ALPHA8,
            GL_RGBA8_SNORM => InternalFormat::RGBA8_SNORM,
            GL_RGB5_A1 => InternalFormat::RGB5_A1,
            GL_RGBA4 => InternalFormat::RGBA4,
            GL_RGB10_A2 => InternalFormat::RGB10_A2,
            GL_RGBA16F => InternalFormat::RGBA16F,
            GL_RGBA32F => InternalFormat::RGBA32F,
            GL_RGBA8UI => InternalFormat::RGBA8UI,
            GL_RGBA8I => InternalFormat::RGBA8I,
            GL_RGB10_A2UI => InternalFormat::RGB10_A2UI,
            GL_RGBA16UI => InternalFormat::RGBA16UI,
            GL_RGBA16I => InternalFormat::RGBA16I,
            GL_RGBA32I => InternalFormat::RGBA32I,
            GL_RGBA32UI => InternalFormat::RGBA32UI,
            GL_DEPTH_COMPONENT16 => InternalFormat::DepthComponent16,
            GL_DEPTH_COMPONENT24 => InternalFormat::DepthComponent24,
            GL_DEPTH_COMPONENT32F => InternalFormat::DepthComponent32F,
            GL_DEPTH24_STENCIL8 => InternalFormat::Depth24Stencil8,
            GL_DEPTH32F_STENCIL8 => InternalFormat::Depth32FStencil8,
            GL_STENCIL_INDEX8 => InternalFormat::StencilIndex8,
            _ => panic!("Invalid GLenum {:?} for InternalFormat", gl_enum),
        }
    }
}
impl<'a> From<&'a InternalFormat> for GLenum {
    #[inline]
    fn from(internal_format: &'a InternalFormat) -> Self {
        match internal_format {
            &InternalFormat::R8 => GL_R8,
            &InternalFormat::R8_SNORM => GL_R8_SNORM,
            &InternalFormat::R16F => GL_R16F,
            &InternalFormat::R32F => GL_R32F,
            &InternalFormat::R8UI => GL_R8UI,
            &InternalFormat::R8I => GL_R8I,
            &InternalFormat::R16UI => GL_R16UI,
            &InternalFormat::R16I => GL_R16I,
            &InternalFormat::R32UI => GL_R32UI,
            &InternalFormat::R32I => GL_R32I,
            &InternalFormat::RG8 => GL_RG8,
            &InternalFormat::RG8_SNORM => GL_RG8_SNORM,
            &InternalFormat::RG16F => GL_RG16F,
            &InternalFormat::RG32F => GL_RG32F,
            &InternalFormat::RG8UI => GL_RG8UI,
            &InternalFormat::RG8I => GL_RG8I,
            &InternalFormat::RG16UI => GL_RG16UI,
            &InternalFormat::RG16I => GL_RG16I,
            &InternalFormat::RG32UI => GL_RG32UI,
            &InternalFormat::RG32I => GL_RG32I,
            &InternalFormat::RGB8 => GL_RGB8,
            &InternalFormat::SRGB8 => GL_SRGB8,
            &InternalFormat::RGB565 => GL_RGB565,
            &InternalFormat::RGB8_SNORM => GL_RGB8_SNORM,
            &InternalFormat::R11F_G11F_B10F => GL_R11F_G11F_B10F,
            &InternalFormat::RGB9_E5 => GL_RGB9_E5,
            &InternalFormat::RGB16F => GL_RGB16F,
            &InternalFormat::RGB32F => GL_RGB32F,
            &InternalFormat::RGB8UI => GL_RGB8UI,
            &InternalFormat::RGB8I => GL_RGB8I,
            &InternalFormat::RGB16UI => GL_RGB16UI,
            &InternalFormat::RGB16I => GL_RGB16I,
            &InternalFormat::RGB32UI => GL_RGB32UI,
            &InternalFormat::RGB32I => GL_RGB32I,
            &InternalFormat::RGBA => GL_RGBA,
            &InternalFormat::RGBA8 => GL_RGBA8,
            &InternalFormat::SRGB8_ALPHA8 => GL_SRGB8_ALPHA8,
            &InternalFormat::RGBA8_SNORM => GL_RGBA8_SNORM,
            &InternalFormat::RGB5_A1 => GL_RGB5_A1,
            &InternalFormat::RGBA4 => GL_RGBA4,
            &InternalFormat::RGB10_A2 => GL_RGB10_A2,
            &InternalFormat::RGBA16F => GL_RGBA16F,
            &InternalFormat::RGBA32F => GL_RGBA32F,
            &InternalFormat::RGBA8UI => GL_RGBA8UI,
            &InternalFormat::RGBA8I => GL_RGBA8I,
            &InternalFormat::RGB10_A2UI => GL_RGB10_A2UI,
            &InternalFormat::RGBA16UI => GL_RGBA16UI,
            &InternalFormat::RGBA16I => GL_RGBA16I,
            &InternalFormat::RGBA32I => GL_RGBA32I,
            &InternalFormat::RGBA32UI => GL_RGBA32UI,
            &InternalFormat::DepthComponent16 => GL_DEPTH_COMPONENT16,
            &InternalFormat::DepthComponent24 => GL_DEPTH_COMPONENT24,
            &InternalFormat::DepthComponent32F => GL_DEPTH_COMPONENT32F,
            &InternalFormat::Depth24Stencil8 => GL_DEPTH24_STENCIL8,
            &InternalFormat::Depth32FStencil8 => GL_DEPTH32F_STENCIL8,
            &InternalFormat::StencilIndex8 => GL_STENCIL_INDEX8,
        }
    }
}
impl From<InternalFormat> for GLenum {
    #[inline(always)]
    fn from(internal_format: InternalFormat) -> Self {
        From::from(&internal_format)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AttributeKind {
    Bool,
    Int,
    Float,
    BoolVec2,
    IntVec2,
    FloatVec2,
    BoolVec3,
    IntVec3,
    FloatVec3,
    BoolVec4,
    IntVec4,
    FloatVec4,
}

impl From<GLenum> for AttributeKind {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_BOOL => AttributeKind::Bool,
            GL_INT => AttributeKind::Int,
            GL_FLOAT => AttributeKind::Float,

            GL_BOOL_VEC2 => AttributeKind::BoolVec2,
            GL_INT_VEC2 => AttributeKind::IntVec2,
            GL_FLOAT_VEC2 => AttributeKind::FloatVec2,

            GL_BOOL_VEC3 => AttributeKind::BoolVec3,
            GL_INT_VEC3 => AttributeKind::IntVec3,
            GL_FLOAT_VEC3 => AttributeKind::FloatVec3,

            GL_BOOL_VEC4 => AttributeKind::BoolVec4,
            GL_INT_VEC4 => AttributeKind::IntVec4,
            GL_FLOAT_VEC4 => AttributeKind::FloatVec4,

            _ => panic!("Invalid GLenum {:?} for AttributeKind", gl_enum),
        }
    }
}

impl<'a> From<&'a AttributeKind> for GLenum {
    #[inline]
    fn from(attribute_kind: &'a AttributeKind) -> Self {
        match attribute_kind {
            &AttributeKind::Bool => GL_BOOL,
            &AttributeKind::Int => GL_INT,
            &AttributeKind::Float => GL_FLOAT,

            &AttributeKind::BoolVec2 => GL_BOOL_VEC2,
            &AttributeKind::IntVec2 => GL_INT_VEC2,
            &AttributeKind::FloatVec2 => GL_FLOAT_VEC2,

            &AttributeKind::BoolVec3 => GL_BOOL_VEC3,
            &AttributeKind::IntVec3 => GL_INT_VEC3,
            &AttributeKind::FloatVec3 => GL_FLOAT_VEC3,

            &AttributeKind::BoolVec4 => GL_BOOL_VEC4,
            &AttributeKind::IntVec4 => GL_INT_VEC4,
            &AttributeKind::FloatVec4 => GL_FLOAT_VEC4,
        }
    }
}
impl From<AttributeKind> for GLenum {
    #[inline(always)]
    fn from(attribute_kind: AttributeKind) -> Self {
        From::from(&attribute_kind)
    }
}

impl AttributeKind {
    #[inline]
    pub fn item_data(&self) -> (usize, DataKind) {
        match self {
            &AttributeKind::Bool => (1, DataKind::Bool),
            &AttributeKind::Int => (1, DataKind::Int),
            &AttributeKind::Float => (1, DataKind::Float),

            &AttributeKind::BoolVec2 => (2, DataKind::Bool),
            &AttributeKind::IntVec2 => (2, DataKind::Int),
            &AttributeKind::FloatVec2 => (2, DataKind::Float),

            &AttributeKind::BoolVec3 => (3, DataKind::Bool),
            &AttributeKind::IntVec3 => (3, DataKind::Int),
            &AttributeKind::FloatVec3 => (3, DataKind::Float),

            &AttributeKind::BoolVec4 => (4, DataKind::Bool),
            &AttributeKind::IntVec4 => (4, DataKind::Int),
            &AttributeKind::FloatVec4 => (4, DataKind::Float),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UniformKind {
    Float,
    Int,

    FloatVec2,
    IntVec2,

    FloatVec3,
    IntVec3,

    FloatVec4,
    IntVec4,

    FloatMat2,
    FloatMat3,
    FloatMat4,

    Sampler2D,
    SamplerExternalOES
}

impl From<GLenum> for UniformKind {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_FLOAT => UniformKind::Float,
            GL_INT => UniformKind::Int,

            GL_FLOAT_VEC2 => UniformKind::FloatVec2,
            GL_INT_VEC2 => UniformKind::IntVec2,

            GL_FLOAT_VEC3 => UniformKind::FloatVec3,
            GL_INT_VEC3 => UniformKind::IntVec3,

            GL_FLOAT_VEC4 => UniformKind::FloatVec4,
            GL_INT_VEC4 => UniformKind::IntVec4,

            GL_FLOAT_MAT2 => UniformKind::FloatMat2,
            GL_FLOAT_MAT3 => UniformKind::FloatMat3,
            GL_FLOAT_MAT4 => UniformKind::FloatMat4,

            GL_SAMPLER_2D => UniformKind::Sampler2D,
            GL_SAMPLER_EXTERNAL_OES => UniformKind::SamplerExternalOES,
            _ => panic!("Invalid GLenum {:?} for UniformKind", gl_enum),
        }
    }
}

impl UniformKind  {
    #[inline]
    pub fn toUniform(&self) -> GLenum {
        match self {
            UniformKind::Sampler2D => GL_TEXTURE_2D,
            UniformKind::SamplerExternalOES => 0x8d65,
            _ => unimplemented!()
        }
    }
}

impl<'a> From<&'a UniformKind> for GLenum {
    #[inline]
    fn from(uniform_kind: &'a UniformKind) -> Self {
        match uniform_kind {
            &UniformKind::Float => GL_FLOAT,
            &UniformKind::Int => GL_INT,

            &UniformKind::FloatVec2 => GL_FLOAT_VEC2,
            &UniformKind::IntVec2 => GL_INT_VEC2,

            &UniformKind::FloatVec3 => GL_FLOAT_VEC3,
            &UniformKind::IntVec3 => GL_INT_VEC3,

            &UniformKind::FloatVec4 => GL_FLOAT_VEC4,
            &UniformKind::IntVec4 => GL_INT_VEC4,

            &UniformKind::FloatMat2 => GL_FLOAT_MAT2,
            &UniformKind::FloatMat3 => GL_FLOAT_MAT3,
            &UniformKind::FloatMat4 => GL_FLOAT_MAT4,

            &UniformKind::Sampler2D => GL_SAMPLER_2D,
            &UniformKind::SamplerExternalOES => 0x8D66,
        }
    }
}
impl From<UniformKind> for GLenum {
    #[inline(always)]
    fn from(uniform_kind: UniformKind) -> Self {
        From::from(&uniform_kind)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Wrap {
    Repeat,
    Clamp,
    MirroredRepeat,
}

impl From<GLenum> for Wrap {
    #[inline]
    fn from(gl_enum: GLenum) -> Self {
        match gl_enum {
            GL_REPEAT => Wrap::Repeat,
            GL_CLAMP_TO_EDGE => Wrap::Clamp,
            GL_MIRRORED_REPEAT => Wrap::MirroredRepeat,
            _ => panic!("Invalid GLenum {:?} for Wrap", gl_enum),
        }
    }
}

impl<'a> From<&'a Wrap> for GLenum {
    #[inline]
    fn from(wrap: &'a Wrap) -> Self {
        match wrap {
            &Wrap::Repeat => GL_REPEAT,
            &Wrap::Clamp => GL_CLAMP_TO_EDGE,
            &Wrap::MirroredRepeat => GL_MIRRORED_REPEAT,
        }
    }
}
impl From<Wrap> for GLenum {
    #[inline(always)]
    fn from(wrap: Wrap) -> Self {
        From::from(&wrap)
    }
}
