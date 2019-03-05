//
//  OpenGLView.h
//  Tutorial01
//
//  Created by kesalin@gmail.com on 12-11-24.
//  Copyright (c) 2012年 http://blog.csdn.net/kesalin/. All rights reserved.
//

#import <UIKit/UIKit.h>
#import <QuartzCore/QuartzCore.h>
#include <OpenGLES/ES2/gl.h>
#include <OpenGLES/ES2/glext.h>

typedef NS_ENUM(NSUInteger, GPUImageFillModeType) {
    kGPUImageFillModeStretch,
    kGPUImageFillModePreserveAspectRatio,
    kGPUImageFillModePreserveAspectRatioAndFill
    
};

typedef NS_ENUM(NSUInteger, GPUImageRotationMode) {
    kGPUImageNoRotation,
    kGPUImageRotateLeft,
    kGPUImageRotateRight,
    kGPUImageFlipVertical,
    kGPUImageFlipHorizonal,
    kGPUImageRotateRightFlipVertical,
    kGPUImageRotateRightFlipHorizontal,
    kGPUImageRotate180
};
@interface OpenGLView : UIView {
    CAEAGLLayer* _eaglLayer;
    EAGLContext* _context;
    GLuint _colorRenderBuffer;
    GLuint _frameBuffer;
    
    GLuint _programHandle;
    GLuint _positionSlot;
    GLuint _inputTextureCoordinateSlot;
    GLuint _inputImageTexture;
}
@property(readwrite, nonatomic) GPUImageFillModeType fillMode;

- (void) setInputImageSize:(CGSize) size;
- (void) setInputImageRotation:(GPUImageRotationMode) rotation;
- (void)setRenderBackingColorWithRed:(CGFloat) red green:(CGFloat)green blue:(CGFloat)blue alpha:(CGFloat)alpha;
//- (id)initWithFrame:(CGRect)frame context:(EAGLContext*) context;
- (instancetype)initWithFrame:(CGRect)frame;
- (instancetype)initWithCoder:(NSCoder *)aDecoder;
- (void)renderTextureId:(GLuint) textureId;
- (CGRect)calculateRenderFrame;

@end
