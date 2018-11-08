use super::GLSize;

#[derive(Copy, Clone, Debug)]
pub enum FillMode {
    stretch,
    preserveAspectRatio,
    preserveAspectRatioAndFill
}

impl FillMode {
    pub fn transformVertices(&self, vertices: [f32;8], fromInputSize : GLSize, toFitSize: GLSize) -> [f32;8] {


        let aspectRatio = (fromInputSize.height as f32) / (fromInputSize.width as f32);
        let targetAspectRatio = (toFitSize.height as f32) / (toFitSize.width as f32);

        let (xRatio, yRatio) =  match self {
            FillMode::stretch => {
                (1.0,1.0)
            },
            FillMode::preserveAspectRatio => {
                if aspectRatio > targetAspectRatio {
                    let x = fromInputSize.width as f32 / toFitSize.width as f32 * ( toFitSize.height as f32 / fromInputSize.height as f32);
                    (x,1.0)
                }else{
                    let y = fromInputSize.height as f32 / toFitSize.height as f32 * ( toFitSize.width as f32 / fromInputSize.width as f32);
                    (1.0,y)
                }
            },
            FillMode::preserveAspectRatioAndFill => {
                if aspectRatio > targetAspectRatio {
                    let y = fromInputSize.height as f32 / toFitSize.height as f32 * (toFitSize.width as f32 / fromInputSize.width as f32);
                    (1.0,y)
                }else {
                    let x = toFitSize.height as f32 / fromInputSize.height as f32 * (fromInputSize.width as f32 / toFitSize.width as f32);
                    (x,1.0)
                }
            }
        };

        let xConversionRatio = xRatio * (toFitSize.width as f32) / 2.0;
        let xConversionDivisor = (toFitSize.width as f32) / 2.0;
        let yConversionRatio = yRatio * (toFitSize.height as f32) / 2.0;
        let yConversionDivisor = (toFitSize.height as f32) / 2.0;

        let value1 = vertices[0] * xConversionRatio / xConversionDivisor;
        let value2 = vertices[1] * yConversionRatio / yConversionDivisor;
        let value3 = vertices[2] * xConversionRatio / xConversionDivisor;
        let value4 = vertices[3] * yConversionRatio / yConversionDivisor;
        let value5 = vertices[4] * xConversionRatio / xConversionDivisor;
        let value6 = vertices[5] * yConversionRatio / yConversionDivisor;
        let value7 = vertices[6] * xConversionRatio / xConversionDivisor;
        let value8 = vertices[7] * yConversionRatio / yConversionDivisor;

        return [value1, value2, value3, value4, value5, value6, value7, value8]
    }
}