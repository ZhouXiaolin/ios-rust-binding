use gles_rust_binding::*;
pub struct Color {
    pub redComponent: f32,
    pub greenComponent: f32,
    pub blueComponent: f32,
    pub alphaComponent: f32
}

impl Color{
    pub fn new(redComponent: f32, greenComponent: f32, blueComponent: f32, alphaComponent: f32) -> Self{
        Color{redComponent:redComponent,greenComponent:greenComponent,blueComponent:blueComponent,alphaComponent:alphaComponent}
    }

    pub fn black() -> Self {
        Color::new(0.0,0.0,0.0,1.0)
    }

    pub fn white() -> Self {
        Color::new(1.0,1.0,1.0,1.0)
    }

    pub fn red() -> Self {
        Color::new(1.0, 0.0, 0.0,1.0)
    }

    pub fn green() -> Self {
        Color::new(0.0,1.0,0.0,1.0)
    }

    pub fn blue() -> Self {
        Color::new(0.0,0.0,1.0,1.0)
    }

    pub fn transparent() -> Self {
        Color::new(0.0,0.0,0.0,0.0)
    }


    pub fn toGLArray(&self) -> [GLfloat;3] {
        [self.redComponent as GLfloat,self.greenComponent as GLfloat,self.blueComponent as GLfloat]
    }

    pub fn toGLArrayWithAlpha(&self) -> [GLfloat;4] {
        [self.redComponent as GLfloat,self.greenComponent as GLfloat,self.blueComponent as GLfloat, self.alphaComponent as GLfloat]
    }

}


