use super::{Position,Size};
#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    noRotation,
    rotateCounterclockwise,
    rotateClockwise,
    rotate180,
    flipHorizontally,
    flipVertically,
    rotateClockwiseAndFlipVertically,
    rotateClockwiseAndFlipHorizontally,
}

impl From<i32> for Rotation {
    fn from(num: i32) -> Rotation {
        match num {
            0 => Rotation::noRotation,
            1 => Rotation::rotateCounterclockwise,
            2 => Rotation::rotateClockwise,
            3 => Rotation::rotate180,
            4 => Rotation::flipHorizontally,
            5 => Rotation::flipVertically,
            6 => Rotation::rotateClockwiseAndFlipVertically,
            7 => Rotation::rotateClockwiseAndFlipHorizontally,
            _ => {
                unreachable!("Error")
            }
        }
    }
}

impl Into<usize> for Rotation {
    fn into(self) -> usize{
        match self {
            Rotation::noRotation => 0,
            Rotation::rotateCounterclockwise => 1,
            Rotation::rotateClockwise => 2,
            Rotation::rotate180 => 3,
            Rotation::flipHorizontally => 4,
            Rotation::flipVertically => 5,
            Rotation::rotateClockwiseAndFlipVertically => 6,
            Rotation::rotateClockwiseAndFlipHorizontally => 7
        }
    }

}



impl Rotation {

    pub fn toRawValue(&self) -> usize {
        match self {
            Rotation::noRotation => 0,
            Rotation::rotateCounterclockwise => 1,
            Rotation::rotateClockwise => 2,
            Rotation::rotate180 => 3,
            Rotation::flipHorizontally => 4,
            Rotation::flipVertically => 5,
            Rotation::rotateClockwiseAndFlipVertically => 6,
            Rotation::rotateClockwiseAndFlipHorizontally => 7
        }
    }
    pub fn flipsDimensions(&self) -> bool {

        match self {
            Rotation::noRotation | Rotation::rotate180 | Rotation::flipHorizontally | Rotation::flipVertically => false,
            _ => true
        }
    }

    pub fn textureCoordinates(&self) -> [f32;8] {

        match self {
            Rotation::noRotation => [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            Rotation::rotateCounterclockwise => [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0],
            Rotation::rotateClockwise =>[1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            Rotation::rotate180 => [1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0],
            Rotation::flipHorizontally => [1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            Rotation::flipVertically => [0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0],
            Rotation::rotateClockwiseAndFlipVertically => [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0],
            Rotation::rotateClockwiseAndFlipHorizontally => [1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        }
    }

    pub fn croppedTextureCoordinates(&self, offsetFromOrigin:Position, cropSize: Size) -> [f32;8] {
        let minX = offsetFromOrigin.x;
        let minY = offsetFromOrigin.y;
        let maxX = offsetFromOrigin.x + cropSize.width;
        let maxY = offsetFromOrigin.y + cropSize.height;

        match self {
            Rotation::noRotation => [minX, minY, maxX, minY, minX, maxY, maxX, maxY],
            Rotation::rotateCounterclockwise => [minX, maxY, minX, minY, maxX, maxY, maxX, minY],
            Rotation::rotateClockwise => [maxX, minY, maxX, maxY, minX, minY, minX, maxY],
            Rotation::rotate180 => [maxX, maxY, minX, maxY, maxX, minY, minX, minY],
            Rotation::flipHorizontally => [maxX, minY, minX, minY, maxX, maxY, minX, maxY],
            Rotation::flipVertically => [minX, maxY, maxX, maxY, minX, minY, maxX, minY],
            Rotation::rotateClockwiseAndFlipVertically => [minX, minY, minX, maxY, maxX, minY, maxX, maxY],
            Rotation::rotateClockwiseAndFlipHorizontally => [maxX, maxY, maxX, minY, minX, maxY, minX, minY],
        }
    }
}