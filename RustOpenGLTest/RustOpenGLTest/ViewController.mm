//
//  ViewController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2018/9/4.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import "ViewController.h"
#import "gpuimage.h"
#import "XLHelpClass.h"
#import <objc/runtime.h>

#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "OpenGLView.h"
#import <GLKit/GLKit.h>
#import "GLESUtils.h"
#import "CameraEntry.h"

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
     gl_FragColor = texture2D(inputImageTexture, textureCoordinate).bgra;
 }
 
 );

@interface ViewController ()<GLKViewDelegate,AVCaptureVideoDataOutputSampleBufferDelegate>
{
    OpenGLView* glView;
    EAGLContext* currentContext;
    
    long g;
    long pic;
    long surface;
    long basic;
    long output;
    long context;
    
    
    GLuint _programHandle;
    GLuint _positionSlot;
    GLuint _inputTextureCoordinateSlot;
    GLuint _inputImageTexture;
    
    GLuint textureId;
    
    
    CameraEntry* cameraEntry;
    
    
    BOOL isFirst;

}
@end

@implementation ViewController

- (void)glkView:(GLKView *)view drawInRect:(CGRect)rect{
   
//     Create program, attach shaders, compile and link program
    
    

    if (output > 0) {
        glUseProgram(_programHandle);
        
        
        
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
    
}


void dataProviderReleaseCallback (void *info, const void *data, size_t size)
{
    free((void *)data);
}

- (void)captureOutput:(AVCaptureOutput *)output didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer fromConnection:(AVCaptureConnection *)connection{
    
    if (isFirst == FALSE) {
        isFirst = TRUE;

        NSLog(@"First");
    }else{
        NSLog(@"DDDDDDD");
    }
    
    NSLog(@"AAAA");
}


- (GLuint) setupTexture{
    NSString* path = [[NSBundle mainBundle] pathForResource:@"IMG_1592" ofType:@"JPG"];
    
    UIImage* image = [[UIImage alloc] initWithContentsOfFile:path];
    CGImage* newImageSource = [image CGImage];
    int width = (int)CGImageGetWidth(newImageSource);
    int height = (int)CGImageGetHeight(newImageSource);
    
    GLubyte *imageData = (GLubyte*)calloc(1, width*height*4);
    CGColorSpaceRef genericRGBColorspace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext = CGBitmapContextCreate(imageData, width, height, 8, width*4, genericRGBColorspace, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext, CGRectMake(0, 0, width, height), newImageSource);
    
    
    GLuint imageTexture = 0;
    glGenTextures(1, &imageTexture);
    glBindTexture(GL_TEXTURE_2D, imageTexture);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_BGRA, GL_UNSIGNED_BYTE, imageData);
    CGContextRelease(imageContext);
    CGColorSpaceRelease(genericRGBColorspace);
    free(imageData);
    newImageSource = nil;
    image = nil;
    path = nil;
    
    return imageTexture;
}

- (void)viewDidLoad {
    [super viewDidLoad];

    self.view.backgroundColor = [UIColor blueColor];
    
//    cameraEntry = [[CameraEntry alloc] initWithSessionPreset:(AVCaptureSessionPresetPhoto) location:(AVCaptureDevicePositionBack) captureAsYUV:FALSE];
//    [cameraEntry setVideoOutputDelegate:self];
//    [cameraEntry startCapture];
    
    currentContext = [[EAGLContext alloc] initWithAPI:kEAGLRenderingAPIOpenGLES2];
    
    glView = [[OpenGLView alloc] initWithFrame:[UIScreen mainScreen].bounds context:currentContext];
    [self.view addSubview:glView];



    [glView render:^GLuint{

        NSString* path = [[NSBundle mainBundle] pathForResource:@"IMG_1592" ofType:@"JPG"];
        
        UIImage* image = [[UIImage alloc] initWithContentsOfFile:path];
        CGImage* newImageSource = [image CGImage];
        int width = (int)CGImageGetWidth(newImageSource);
        int height = (int)CGImageGetHeight(newImageSource);
        
        textureId = [XLHelpClass createTexture:image];
        
        
        g = xhey_init_graph();
        context = init_context();
        pic = xhey_init_picture_textureId(textureId, width, height, 0);
        
        basic = xhey_init_basic_filter(context);
        output = xhey_init_picture_output(context, width, height, 0);
        xhey_picture_graph(g, pic, basic, 0, 0, 0, 0, output);
        xhey_graph_forward(g);

//        UIImage* result = [XLHelpClass readImageFromFBO:width height:height];
//        glBindFramebuffer(GL_FRAMEBUFFER, 0);
//
//        textureId = [XLHelpClass createTexture:result];
        return xhey_picture_output_get_texture_id(output);
    }];
    
    
    


    
//    [EAGLContext setCurrentContext:currentContext];
    
    


//    [EAGLContext setCurrentContext:nil];
//
//    currentContext = nil;
    
    
    UIButton* button = [UIButton buttonWithType:UIButtonTypeCustom];
    button.frame = CGRectMake(0, 0, 50, 50);
    button.center = self.view.center;
    button.backgroundColor = [UIColor redColor];
    [button addTarget:self action:@selector(click) forControlEvents:UIControlEventTouchUpInside];
    [self.view addSubview:button];
    

    
}
- (void) click
{
    NSLog(@"TTTTTTTTTTTT");
    glDeleteTextures(1, &textureId);
    release_context(context);
    release_picture(pic);
    release_basic_filter(basic);
    release_graph(g);
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
