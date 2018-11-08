use super::Rotation;
#[derive(Copy, Clone, Debug)]
pub enum ImageOrientation{
    portrait,
    portraitUpsideDown,
    landscapeLeft,
    landscapeRight
}

impl From<i32> for ImageOrientation {
    fn from(num: i32) -> ImageOrientation {
        match num {
            0 => ImageOrientation::portrait,
            1 => ImageOrientation::portraitUpsideDown,
            2 => ImageOrientation::landscapeLeft,
            3 => ImageOrientation::landscapeRight,
            _ => unreachable!("error")
        }
    }
}


impl ImageOrientation {

    pub fn rotationNeededForOrientation(&self, targetOrientation: ImageOrientation) -> Rotation {
        match (self,targetOrientation) {
            (ImageOrientation::portrait, ImageOrientation::portrait) | (ImageOrientation::portraitUpsideDown, ImageOrientation::portraitUpsideDown)
            | (ImageOrientation::landscapeLeft,ImageOrientation::landscapeLeft) | (ImageOrientation::landscapeRight,ImageOrientation::landscapeRight) => Rotation::noRotation,

            (ImageOrientation::portrait, ImageOrientation::portraitUpsideDown)  |  (ImageOrientation::portraitUpsideDown, ImageOrientation::portrait)
            | (ImageOrientation::landscapeLeft, ImageOrientation::landscapeRight) | (ImageOrientation::landscapeRight, ImageOrientation::landscapeLeft) => Rotation::rotate180,

            (ImageOrientation::portrait, ImageOrientation::landscapeLeft) | (ImageOrientation::landscapeRight, ImageOrientation::portrait)
            | (ImageOrientation::landscapeLeft, ImageOrientation::portraitUpsideDown) | (ImageOrientation::portraitUpsideDown, ImageOrientation::landscapeRight) => Rotation::rotateCounterclockwise,

            (ImageOrientation::landscapeRight, ImageOrientation::portraitUpsideDown) | (ImageOrientation::landscapeLeft, ImageOrientation::portrait)
            | (ImageOrientation::portrait, ImageOrientation::landscapeRight) | (ImageOrientation::portraitUpsideDown, ImageOrientation::landscapeLeft) => Rotation::rotateClockwise
        }
    }
}
