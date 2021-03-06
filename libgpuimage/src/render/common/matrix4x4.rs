use gles_rust_binding::*;
type Float = f32;
#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix4x4 {
    m11:Float, m12:Float, m13:Float, m14:Float,
    m21:Float, m22:Float, m23:Float, m24:Float,
    m31:Float, m32:Float, m33:Float, m34:Float,
    m41:Float, m42:Float, m43:Float, m44:Float,
}

impl Matrix4x4{
    pub fn new(rowMajorValues:[f32;16]) -> Self {
        Matrix4x4{
            m11:rowMajorValues[0], m12:rowMajorValues[1], m13:rowMajorValues[2], m14:rowMajorValues[3],
            m21:rowMajorValues[4], m22:rowMajorValues[5], m23:rowMajorValues[6], m24:rowMajorValues[7],
            m31:rowMajorValues[8], m32:rowMajorValues[9], m33:rowMajorValues[10], m34:rowMajorValues[11],
            m41:rowMajorValues[12], m42:rowMajorValues[13], m43:rowMajorValues[14], m44:rowMajorValues[15],
        }
    }

    pub fn identity() -> Self {
        Self::new([0.0;16])
    }

    pub fn toRowMajorGLArray(&self) -> [GLfloat;16] {
       return [self.m11, self.m12, self.m13, self.m14,
               self.m21, self.m22, self.m23, self.m24,
               self.m31, self.m32, self.m33, self.m34,
               self.m41, self.m42, self.m43, self.m44];
    }
}