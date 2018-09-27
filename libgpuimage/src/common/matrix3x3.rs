type Float = f32;

#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix3x3 {
    m11:Float, m12:Float, m13:Float,
    m21:Float, m22:Float, m23:Float,
    m31:Float, m32:Float, m33:Float,
}

impl Matrix3x3{
    fn new(rowMajorValues:[f32;9]) -> Self {
        Matrix3x3{
            m11:rowMajorValues[0], m12:rowMajorValues[1], m13:rowMajorValues[2],
            m21:rowMajorValues[3], m22:rowMajorValues[4], m23:rowMajorValues[5],
            m31:rowMajorValues[6], m32:rowMajorValues[7], m33:rowMajorValues[8],
        }
    }

    fn identity() -> Self {
        Self::new([1.0,0.0,0.0,
                                0.0,1.0,0.0,
                                0.0,0.0,1.0])
    }
    fn centerOnly() -> Self {
        Self::new([0.0,0.0,0.0,
                                0.0,1.0,0.0,
                                0.0,0.0,0.0])
    }
}