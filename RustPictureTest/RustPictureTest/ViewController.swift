//
//  ViewController.swift
//  RustPictureTest
//
//  Created by 周晓林 on 2018/11/12.
//  Copyright © 2018 Solaren. All rights reserved.
//

import UIKit
import GLKit


let kVertexString = """

attribute vec4 inputTextureCoordinate;
varying vec2 textureCoordinate;
void main(){
    gl_Position = position;
    textureCoordinate = inputTextureCoordinate.xy;
}

"""

let kFragmentString = """

precision mediump float;

varying highp vec2 textureCoordinate;
uniform sampler2D inputImageTexture;

void main()
{
    gl_FragColor = texture2D(inputImageTexture, textureCoordinate);
}

"""




class ViewController: UIViewController {

    var glView : GLKView!
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        let currentContext = EAGLContext(api: .openGLES2)
        
        
        let path = Bundle.main.path(forResource: "IMG_1592", ofType: "JPG")
        let image = UIImage(contentsOfFile: path!)
        let width = (image?.cgImage?.width)!;
        let height = (image?.cgImage?.height)!;
        var imageData = UnsafeMutablePointer<GLubyte>.allocate(capacity:Int(width * height) * 4)
        
        let genericRGBColorspace = CGColorSpaceCreateDeviceRGB()

       
        let imageContext = CGContext(data:imageData, width:Int(width), height:Int(height), bitsPerComponent:8, bytesPerRow:Int(width) * 4, space:genericRGBColorspace,  bitmapInfo:CGImageAlphaInfo.premultipliedFirst.rawValue | CGBitmapInfo.byteOrder32Little.rawValue)
        
        imageContext?.draw((image?.cgImage)!, in:CGRect(x:0.0, y:0.0, width:CGFloat(width), height:CGFloat(height)))
        
        glView = GLKView(frame: UIScreen.main.bounds, context: currentContext!)
        glView.delegate = self
        self.view.addSubview(glView)
        
        EAGLContext.setCurrent(currentContext)
        
        
    
        let g = xhey_init_graph();
        
        let pic = xhey_init_picture(imageData, Int32(width), Int32(height))
        imageData.deallocate()
        
        let basic = xhey_init_basic_filter()
        
        let output = xhey_init_picture_output(Int32(width), Int32(height))
        
        xhey_graph(g, pic, basic, nil, output)
        
        xhey_graph_forward(g)
        
        
        
        // Do any additional setup after loading the view, typically from a nib.
    }


}


extension ViewController : GLKViewDelegate {
    func glkView(_ view: GLKView, drawIn rect: CGRect) {
        glClearColor(1.0, 0.0, 0.0, 1.0);
        glClear(GLbitfield(GL_COLOR_BUFFER_BIT));
    }
}

