use core::*;
use core::framebuffer::{Framebuffer,ImageOrientation,GLSize};

use std::mem;
use std::cell::{RefCell,Cell};
use gles_rust_binding::*;

#[repr(C)]
pub struct XHeyBasicFilter<'a>{
    _node : RenderNode,
    _targets: RefCell<Vec<Box<&'a dyn Consumer>>>,
    _shader : GLProgram,
    _maximumInputs : i32,
    _inputFramebuffers:RefCell<Vec<Framebuffer>>,
    _renderFramebuffer: RefCell<Framebuffer>,

}



impl<'a> XHeyBasicFilter<'a> {
    pub fn new_shader(vertex:&str,fragment:&str, numberOfInputs: i32) -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let shader = GLProgram::new(vertex,fragment);
        XHeyBasicFilter{
            _node:RenderNode::new(NodeType::BasicFilter),
            _targets:RefCell::new(Vec::new()),
            _maximumInputs:numberOfInputs,
            _shader: shader,
            _inputFramebuffers:RefCell::default(),
            _renderFramebuffer: RefCell::default(),
        }
    }
    pub fn new() -> Self {
        sharedImageProcessingContext.makeCurrentContext();
        let vertexStr = r#"
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;

 varying vec2 textureCoordinate;

 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
    "#;

        let fragmentStr = r#"
 precision mediump float;

 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;

 void main()
 {
     vec4 color = texture2D(inputImageTexture, textureCoordinate);
     gl_FragColor = vec4(color.r, 0.0, 0.0, 1.0);
 }
    "#;
        let shader = GLProgram::new(vertexStr,fragmentStr);

        XHeyBasicFilter{
            _node:RenderNode::new(NodeType::BasicFilter),
            _targets:RefCell::new(Vec::new()),
            _maximumInputs:1,
            _shader: shader,
            _inputFramebuffers: RefCell::default(),
            _renderFramebuffer: RefCell::default()
        }
    }


    pub fn renderFrame(&self){



    }

    fn sizeOfInitialStageBasedOnFramebuffer(&self, inputFramebuffer: &Framebuffer) -> GLSize {
        inputFramebuffer.sizeForTargetOrientation(ImageOrientation::portrait)
    }
}

impl<'a> Node for XHeyBasicFilter<'a>{
    fn get_type_name(&self) -> NodeType {
        NodeType::BasicFilter
    }
}



impl<'a> Source<'a> for XHeyBasicFilter<'a> {
    fn add_target(&self, target: &'a dyn Consumer, _location: u32) {
        let mut targets = self._targets.borrow_mut();
        targets.push(Box::new(target));
        target.set_source(self,_location);
    }

    fn remove_all_targets(&self){

    }
    fn updateTargetsWithFramebuffer(&self, framebuffer:&Framebuffer){
        for (index,target) in self._targets.borrow_mut().iter().enumerate() {
            target.newFramebufferAvailable(framebuffer,index);
        }
    }
}

impl<'a> Consumer for XHeyBasicFilter<'a> {
    fn set_source(&self, _source: &dyn Source, _location: u32) {

    }

    fn newFramebufferAvailable(&self, framebuffer: &Framebuffer, fromSourceIndex: usize){


        let mut inputFramebuffers = self._inputFramebuffers.borrow_mut();


        inputFramebuffers.insert(fromSourceIndex,framebuffer.clone());

        let len = inputFramebuffers.len();

        if len >= self._maximumInputs as usize {




            println!("basicFilter newFramebufferAvailable  1");

            let inputFramebuffer = inputFramebuffers.first().unwrap();
            println!("basicFilter newFramebufferAvailable  2");

            let size = self.sizeOfInitialStageBasedOnFramebuffer(inputFramebuffer);
            println!("basicFilter newFramebufferAvailable  3");

            let renderFramebuffer = sharedImageProcessingContext.frameubfferCache.requestFramebufferWithDefault(ImageOrientation::portrait,size,false);
            println!("basicFilter newFramebufferAvailable  4");

            renderFramebuffer.activateFramebufferForRendering();
            println!("basicFilter newFramebufferAvailable  5");

            clearFramebufferWithColor(Color::black());
            println!("basicFilter newFramebufferAvailable  6");

            renderQuadWithShader(&self._shader,inputFramebuffer);
            println!("basicFilter newFramebufferAvailable  7");




//            let outputFramebuffer = self._renderFramebuffer.borrow();
            self.updateTargetsWithFramebuffer(&renderFramebuffer)
        }

    }

}


#[allow(non_snake_case, unused_variables, dead_code)]
#[no_mangle]
pub extern "C" fn xhey_init_basic_filter<'a>() -> *mut XHeyBasicFilter<'a> {
    let filter = Box::new(XHeyBasicFilter::new());
    Box::into_raw(filter)
}