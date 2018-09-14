//
//  RDHelpClass.m
//  XHGPUImageDemo
//
//  Created by 周晓林 on 2018/6/13.
//  Copyright © 2018年 michael.laifx. All rights reserved.
//

#import "XLHelpClass.h"
#include <sys/utsname.h>
#import <CoreImage/CoreImage.h>
#import <UIKit/UIKit.h>
#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
@implementation XLHelpClass

+(GLuint)loadShader:(GLenum)type withFilepath:(NSString *)shaderFilepath
{
    NSError* error;
    NSString* shaderString = [NSString stringWithContentsOfFile:shaderFilepath
                                                       encoding:NSUTF8StringEncoding
                                                          error:&error];
    if (!shaderString) {
        NSLog(@"Error: loading shader file: %@ %@", shaderFilepath, error.localizedDescription);
        return 0;
    }
    
    return [self loadShader:type withString:shaderString];
}

+(GLuint)loadShader:(GLenum)type withString:(NSString *)shaderString
{
    // Create the shader object
    GLuint shader = glCreateShader(type);
    if (shader == 0) {
        NSLog(@"Error: failed to create shader.");
        return 0;
    }
    
    // Load the shader source
    const char * shaderStringUTF8 = [shaderString UTF8String];
    glShaderSource(shader, 1, &shaderStringUTF8, NULL);
    
    // Compile the shader
    glCompileShader(shader);
    
    // Check the compile status
    GLint compiled = 0;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &compiled);
    
    if (!compiled) {
        GLint infoLen = 0;
        glGetShaderiv ( shader, GL_INFO_LOG_LENGTH, &infoLen );
        
        if (infoLen > 1) {
            char * infoLog = malloc(sizeof(char) * infoLen);
            glGetShaderInfoLog (shader, infoLen, NULL, infoLog);
            NSLog(@"Error compiling shader:\n%s\n", infoLog );
            
            free(infoLog);
        }
        
        glDeleteShader(shader);
        return 0;
    }
    
    return shader;
}
+(GLuint)loadProgram:(NSString*)vertexShaderString withFragmentShaderString:(NSString*)fragmentShaderString
{
    GLuint vertexShader = [self loadShader:GL_VERTEX_SHADER
                                withString:vertexShaderString];
    if (vertexShader == 0)
        return 0;
    
    GLuint fragmentShader = [self loadShader:GL_FRAGMENT_SHADER
                                  withString:fragmentShaderString];
    if (fragmentShader == 0) {
        glDeleteShader(vertexShader);
        return 0;
    }
    
    // Create the program object
    GLuint programHandle = glCreateProgram();
    if (programHandle == 0)
        return 0;
    
    glAttachShader(programHandle, vertexShader);
    glAttachShader(programHandle, fragmentShader);
    
    // Link the program
    glLinkProgram(programHandle);
    
    // Check the link status
    GLint linked;
    glGetProgramiv(programHandle, GL_LINK_STATUS, &linked);
    
    if (!linked) {
        GLint infoLen = 0;
        glGetProgramiv(programHandle, GL_INFO_LOG_LENGTH, &infoLen);
        
        if (infoLen > 1){
            char * infoLog = malloc(sizeof(char) * infoLen);
            glGetProgramInfoLog(programHandle, infoLen, NULL, infoLog);
            
            NSLog(@"Error linking program:\n%s\n", infoLog);
            
            free(infoLog);
        }
        
        glDeleteProgram(programHandle );
        return 0;
    }
    
    // Free up no longer needed shader resources
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);
    
    return programHandle;
    
}
+ (NSString *)pathInCacheDirectory:(NSString *)fileName
{
    //获取沙盒中缓存文件目录
    NSString *cacheDirectory = NSSearchPathForDirectoriesInDomains(NSCachesDirectory, NSUserDomainMask, YES)[0];
    //将传入的文件名加在目录路径后面并返回
    return [cacheDirectory stringByAppendingPathComponent:fileName];
}

+ (NSString*) pathBundlePath
{
    NSString* bundlePath = [[NSBundle mainBundle].resourcePath stringByAppendingPathComponent:@"b612.bundle"];

    return bundlePath;
    
}
+ (CVPixelBufferRef)copyRenderedPixelBuffer:(CVPixelBufferRef)pixelBuffer
{
    const int kBytesPerPixel = 4;
    
    CVPixelBufferLockBaseAddress( pixelBuffer, 0 );
    
    int bufferWidth = (int)CVPixelBufferGetWidth( pixelBuffer );
    int bufferHeight = (int)CVPixelBufferGetHeight( pixelBuffer );
    
    
    size_t bytesPerRow = CVPixelBufferGetBytesPerRow( pixelBuffer );
    uint8_t *baseAddress = CVPixelBufferGetBaseAddress( pixelBuffer );
    
    for ( int row = 0; row < bufferHeight; row += 4 )
    {
        uint8_t *pixel = baseAddress + row * bytesPerRow;
        for ( int column = 0; column < bufferWidth; column += 4 )
        {
            pixel[0] = 255.0;
            pixel[1] = 0; // De-green (second pixel in BGRA is green)
            pixel[2] = 0;
            //            pixel[3] = 0;
            pixel += kBytesPerPixel;
        }
    }
    
    CVPixelBufferUnlockBaseAddress( pixelBuffer, 0 );
    
    return (CVPixelBufferRef)CFRetain( pixelBuffer );
}
+ (BOOL) isLowerThaniPhone6
{
    struct utsname systemInfo;
    int ret = 0;
    ret = uname(&systemInfo);
    NSString* machine = [NSString stringWithUTF8String:systemInfo.machine];
    if ([machine hasPrefix:@"iPhone7"]) {
        return TRUE;
    }
    if ([machine hasPrefix:@"iPhone6"]) {
        return TRUE;
    }
    if ([machine hasPrefix:@"iPhone5"]) {
        return TRUE;
    }
    if ([machine hasPrefix:@"iPad"]) {
        return TRUE;
    }
    
    return FALSE;
}

+ (UIImage *)photoFromPixelBuffer:(CVPixelBufferRef)pixelBufferRef
{
    CVPixelBufferLockBaseAddress(pixelBufferRef, 0);
    
    float width = CVPixelBufferGetWidth(pixelBufferRef);
    float height = CVPixelBufferGetHeight(pixelBufferRef);
    
    CIImage *ciImage = [CIImage imageWithCVPixelBuffer:pixelBufferRef];
    CIContext *temporaryContext = [CIContext contextWithOptions:nil];
    
    CGImageRef videoImage = [temporaryContext
                             createCGImage:ciImage
                             fromRect:CGRectMake(0, 0,
                                                 width,
                                                 height)];
    
    UIImage *image = [UIImage imageWithCGImage:videoImage scale:1.0 orientation:(UIImageOrientationRight)];
    //    UIImage *image = [UIImage imageWithCGImage:videoImage];
    CGImageRelease(videoImage);
    CVPixelBufferUnlockBaseAddress(pixelBufferRef, 0);
    return image;
}
@end
