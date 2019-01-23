//
//  RDHelpClass.h
//  XHGPUImageDemo
//
//  Created by 周晓林 on 2018/6/13.
//  Copyright © 2018年 michael.laifx. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#define UIColorFromRGB(rgbValue) [UIColor colorWithRed:((float)((rgbValue & 0xFF0000) >> 16))/255.0 green:((float)((rgbValue & 0xFF00) >> 8))/255.0 blue:((float)(rgbValue & 0xFF))/255.0 alpha:1.0]
@interface XLHelpClass : NSObject
+ (GLuint) setupTexture: (UIImage*)image;
+ (NSString *)pathInCacheDirectory:(NSString *)fileName;
+ (NSString*) pathBundlePath;
+ (BOOL) isLowerThaniPhone6;
+ (CVPixelBufferRef)copyRenderedPixelBuffer:(CVPixelBufferRef)pixelBuffer;
+ (UIImage *)photoFromPixelBuffer:(CVPixelBufferRef)pixelBufferRef;
+ (GLuint)loadProgram:(NSString*)vertexShaderString withFragmentShaderString:(NSString*)fragmentShaderString;
+ (int ) createTexture: (UIImage*) mBitmap;
+ (UIImage*) readImageFromFBO: (int) width height:(int) height;

@end
