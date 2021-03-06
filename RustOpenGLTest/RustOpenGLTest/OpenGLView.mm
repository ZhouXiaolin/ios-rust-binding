//
//  OpenGLView.m
//  Tutorial01
//
//  Created by kesalin@gmail.com on 12-11-24.
//  Copyright (c) 2012年 http://blog.csdn.net/kesalin/. All rights reserved.
//

#import "OpenGLView.h"
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

// 使用匿名 category 来声明私有成员
@interface OpenGLView()
{
    GLuint imageTexture;
}
- (void)setupLayer;
- (void)setupContext;
- (void)setupProgram;

- (void)setupBuffers;
- (void)destoryBuffers;

- (void)render;

@end

@implementation OpenGLView

+ (Class)layerClass {
    // 只有 [CAEAGLLayer class] 类型的 layer 才支持在其上描绘 OpenGL 内容。
    return [CAEAGLLayer class];
}

- (void)setupLayer
{
    
    _eaglLayer = (CAEAGLLayer*) self.layer;
    
    // CALayer 默认是透明的，必须将它设为不透明才能让其可见
    _eaglLayer.opaque = YES;
    
    // 设置描绘属性，在这里设置不维持渲染内容以及颜色格式为 RGBA8
    _eaglLayer.drawableProperties = [NSDictionary dictionaryWithObjectsAndKeys:
                                    [NSNumber numberWithBool:NO], kEAGLDrawablePropertyRetainedBacking, kEAGLColorFormatRGBA8, kEAGLDrawablePropertyColorFormat, nil];
}

- (void)setupContext {
    // 指定 OpenGL 渲染 API 的版本，在这里我们使用 OpenGL ES 2.0 
    EAGLRenderingAPI api = kEAGLRenderingAPIOpenGLES2;
    _context = [[EAGLContext alloc] initWithAPI:api];
    if (!_context) {
        NSLog(@"Failed to initialize OpenGLES 2.0 context");
        exit(1);
    }
    
    // 设置为当前上下文
    if (![EAGLContext setCurrentContext:_context]) {
        _context = nil;
        NSLog(@"Failed to set current OpenGL context");
        exit(1);
    }
    

}



- (void)setupBuffers
{
    glGenRenderbuffers(1, &_colorRenderBuffer);
    // 设置为当前 renderbuffer
    glBindRenderbuffer(GL_RENDERBUFFER, _colorRenderBuffer);
    // 为 color renderbuffer 分配存储空间
    

    [_context renderbufferStorage:GL_RENDERBUFFER fromDrawable:_eaglLayer];
    
    glViewport(0, 0, self.frame.size.width, self.frame.size.height);

    
    glGenFramebuffers(1, &_frameBuffer);
    // 设置为当前 framebuffer
    glBindFramebuffer(GL_FRAMEBUFFER, _frameBuffer);
    // 将 _colorRenderBuffer 装配到 GL_COLOR_ATTACHMENT0 这个装配点上
    glFramebufferRenderbuffer(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, 
                              GL_RENDERBUFFER, _colorRenderBuffer);
}

- (void)destoryBuffers
{
    glDeleteRenderbuffers(1, &_colorRenderBuffer);
    _colorRenderBuffer = 0;

    glDeleteFramebuffers(1, &_frameBuffer);
    _frameBuffer = 0;
}


- (void)render
{
    
    [EAGLContext setCurrentContext:_context];
    
    
    
    NSString* path = [[NSBundle mainBundle] pathForResource:@"IMG_1592" ofType:@"JPG"];
    
    UIImage* image = [[UIImage alloc] initWithContentsOfFile:path];
    CGImage* newImageSource = [image CGImage];
    int width = (int)CGImageGetWidth(newImageSource);
    int height = (int)CGImageGetHeight(newImageSource);
    
    GLubyte *imageData = (GLubyte*)calloc(1, width*height*4);
    CGColorSpaceRef genericRGBColorspace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext = CGBitmapContextCreate(imageData, width, height, 8, width*4, genericRGBColorspace, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext, CGRectMake(0, 0, width, height), newImageSource);
   
    
    
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
    glBindTexture(GL_TEXTURE_2D,imageTexture);
    glUniform1i(0,_inputImageTexture);
    // Draw triangle
    //
    glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
        
    [_context presentRenderbuffer:GL_RENDERBUFFER];
    
    
    
}

- (id)initWithFrame:(CGRect)frame
{
    self = [super initWithFrame:frame];
    if (self) {
        [self setupLayer];        
        [self setupContext];
//        [self setupProgram];
    }

    return self;
}

- (void)layoutSubviews
{
    [EAGLContext setCurrentContext:_context];

    [self destoryBuffers];
    
    [self setupBuffers];

    [self render];
}

/*
// Only override drawRect: if you perform custom drawing.
// An empty implementation adversely affects performance during animation.
- (void)drawRect:(CGRect)rect
{
    // Drawing code
}
*/

- (void)dealloc{
    [self destoryBuffers];
    glDeleteTextures(1, &imageTexture);
}

@end
