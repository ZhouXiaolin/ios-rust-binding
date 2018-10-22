//
//  ViewController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2018/9/4.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import "ViewController.h"
#import "gpuimage.h"
#import <objc/runtime.h>
#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "OpenGLView.h"
#import <GLKit/GLKit.h>
#import "GLESUtils.h"

#define STRINGIZE(x) #x
#define STRINGIZE2(x) STRINGIZE(x)
#define SHADER_STRING(text) @ STRINGIZE2(text)


NSString* const kVertexString = SHADER_STRING
(
 attribute vec4 position;
 attribute vec4 inputTextureCoordinate;
 
 varying vec2 textureCoordinate;
 
 void main()
 {
     gl_Position = position;
     textureCoordinate = inputTextureCoordinate.xy;
 }
 
 );

NSString* const kFragmentString = SHADER_STRING
(
 precision mediump float;
 
 varying highp vec2 textureCoordinate;
 uniform sampler2D inputImageTexture;
 
 void main()
 {
     gl_FragColor = texture2D(inputImageTexture, textureCoordinate);
 }
 
 );

@interface ViewController ()<GLKViewDelegate>
{
    GLKView* glView;
    EAGLContext* currentContext;
    
    Graph* g;
    XheyPicture* pic;
    XheySurfaceView* surface;
    XheyBasicFilter* basic;
    XheyPictureOutput* output;
    
    
    GLuint _programHandle;
    GLuint _positionSlot;
    GLuint _inputTextureCoordinateSlot;
    GLuint _inputImageTexture;

}
@end

@implementation ViewController

- (void)glkView:(GLKView *)view drawInRect:(CGRect)rect{
   
    // Create program, attach shaders, compile and link program
    //
    _programHandle = [GLESUtils loadProgramString:kVertexString withFragmentShaderString:kFragmentString];
    if (_programHandle == 0) {
        NSLog(@" >> Error: Failed to setup program.");
        return;
    }
    
    glUseProgram(_programHandle);
    
    // Get attribute slot from program
    //
    _positionSlot = glGetAttribLocation(_programHandle, "position");
    _inputTextureCoordinateSlot = glGetAttribLocation(_programHandle, "inputTextureCoordinate");
    _inputImageTexture = glGetUniformLocation(_programHandle, "inputImageTexture");
    
    glClearColor(0, 1.0, 0, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    
    // Setup viewport
    //
    
    GLfloat vertices[] = {
        -1.0,1.0,1.0,1.0,-1.0,-1.0,1.0,-1.0 };
    GLfloat textureCoordinates[] = {
        1.0,1.0, 1.0,0.0, 0.0,1.0, 0.0,0.0
    };
    
    // Load the vertex data
    //
    glVertexAttribPointer(_positionSlot, 2, GL_FLOAT, GL_FALSE, 0, vertices );
    glEnableVertexAttribArray(_positionSlot);
    
    glVertexAttribPointer(_inputTextureCoordinateSlot, 2, GL_FLOAT, GL_FALSE, 0, textureCoordinates);
    glEnableVertexAttribArray(_inputTextureCoordinateSlot);
    
    
    
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D,xhey_picture_output_get_texture_id(output));
    glUniform1i(0,_inputImageTexture);
    // Draw triangle
    //
    glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
    
}


void dataProviderReleaseCallback (void *info, const void *data, size_t size)
{
    free((void *)data);
}


- (void)viewDidLoad {
    [super viewDidLoad];

    self.view.backgroundColor = [UIColor blueColor];
    

    currentContext = [[EAGLContext alloc] initWithAPI:kEAGLRenderingAPIOpenGLES2];
    
    glView = [[GLKView alloc] initWithFrame:[UIScreen mainScreen].bounds context:currentContext];
    glView.delegate = self;
    [self.view addSubview:glView];
    
    
    
    NSString* path1 = [[NSBundle mainBundle] pathForResource:@"IMG_1592" ofType:@"JPG"];

    UIImage* image1 = [[UIImage alloc] initWithContentsOfFile:path1];
    CGImage* newImageSource1 = [image1 CGImage];
    int width1 = (int)CGImageGetWidth(newImageSource1);
    int height1 = (int)CGImageGetHeight(newImageSource1);

    GLubyte *imageData1 = (GLubyte*)calloc(1, width1*height1*4);
    CGColorSpaceRef genericRGBColorspace1 = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext1 = CGBitmapContextCreate(imageData1, width1, height1, 8, width1*4, genericRGBColorspace1, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext1, CGRectMake(0, 0, width1, height1), newImageSource1);
    CGContextRelease(imageContext1);
    CGColorSpaceRelease(genericRGBColorspace1);

    
    [EAGLContext setCurrentContext:currentContext];
    g = xhey_init_graph();
    pic = xhey_init_picture(imageData1, width1, height1);
    free(imageData1);
    basic = xhey_init_basic_filter_2();
//    surface = xhey_init_surface_view(720, 720);
    output = xhey_init_picture_output(width1, height1);
    xhey_graph(g, pic, basic, nullptr, output);
    
    
    xhey_graph_forward(g);
    
//    CGSize _size = CGSizeMake(width1, height1);
//    
//    NSUInteger totalBytesForImage = (int)_size.width * (int)_size.height * 4;
//    
//    GLubyte *rawImagePixels;
//    
//    CGDataProviderRef dataProvider = NULL;
//    
//    rawImagePixels = (GLubyte *)malloc(totalBytesForImage);
//    glReadPixels(0, 0, (int)_size.width, (int)_size.height, GL_RGBA, GL_UNSIGNED_BYTE, rawImagePixels);
//    dataProvider = CGDataProviderCreateWithData(NULL, rawImagePixels, totalBytesForImage, dataProviderReleaseCallback);
//    
//    CGColorSpaceRef defaultRGBColorSpace = CGColorSpaceCreateDeviceRGB();
//    
//    CGImageRef cgImageFromBytes = CGImageCreate((int)_size.width, (int)_size.height, 8, 32, 4 * (int)_size.width, defaultRGBColorSpace, kCGBitmapByteOrderDefault | kCGImageAlphaLast, dataProvider, NULL, NO, kCGRenderingIntentDefault);
//    
//    UIImage *finalImage = [UIImage imageWithCGImage:cgImageFromBytes scale:1.0 orientation:UIImageOrientationUp];
//    
//    
//    int i = 0;
    [EAGLContext setCurrentContext:nil];
    

    
    UIButton* button = [UIButton buttonWithType:UIButtonTypeCustom];
    button.frame = CGRectMake(0, 0, 50, 50);
    button.center = self.view.center;
    button.backgroundColor = [UIColor blueColor];
    [button addTarget:self action:@selector(click) forControlEvents:UIControlEventTouchUpInside];
    [self.view addSubview:button];
    

    
}
- (void) click
{
    NSLog(@"TTTTTTTTTTTT");
    xhey_context_release();
    xhey_release_picture(pic);
    xhey_release_basic_filter(basic);
    xhey_release_graph(g);
    currentContext = nil;
    [EAGLContext setCurrentContext:nil];
    [glView removeFromSuperview];
    glView = nil;
    
}



- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
