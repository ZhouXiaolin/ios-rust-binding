//
//  FilterController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2019/1/22.
//  Copyright © 2019 Solaren. All rights reserved.
//

#import "XHFilterController.h"
#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "gpuimage.h"
#import "XLHelpClass.h"
#import "CameraEntry.h"
#import "OpenGLView.h"
#import "MovieWriter.h"
#import "GPUImageContext.h"
struct Context{
    XHFilterController* self;
};

@implementation WaterViewInfo
@end

@interface XHFilterController()
{
    EAGLContext* currentContext;
    
    long g;
    
    
    
    long render_pic;
    long cam;
    
    long lut;
    long pic;
    
    long surface;
    long basic;
    long output;
    
    long context;
    
    long context_watermark_ptr;
    long watermark_graph;
    long watermark_picture_ptr;
    
    GLuint y_textureId;
    GLuint uv_textureId;
    GLuint textureId;
    GLuint lookup_textureId;
    
    BOOL isFirst;
    
    BOOL update;
    
    Context* ctxt;

    NSString* lut_path;
    BOOL lutUpdate;
    
    
    BOOL isPhoto;

}
@property (nonatomic, strong) CameraEntry* cameraEntry;
@property (nonatomic, strong) OpenGLView* glView;
@property (nonatomic, strong) MovieWriter* movieWriter;
@property (nonatomic, strong) UIImage* renderPicture;
@end
@implementation XHFilterController
#define aw_stride(wid) ((wid % 16 != 0) ? ((wid) + 16 - (wid) % 16): (wid))
void print1(void* context){
    NSLog(@"TTTTTTTTTTTTT --------");
}

void print_test1(void* context){
    Context* ctxt = (Context*)context;
    
}

- (instancetype) initWithPicture:(UIImage*) image
       renderView:(OpenGLView*)glView
{
    self =[super init];
    if (!self) {
        return nil;
    }
    
    isPhoto = true;
    
    
    self.glView = glView;
    self.renderPicture = image;
    
    
    g = xhey_init_graph();
    context = init_context();
    
    
    int render_pic_texture_id = [XLHelpClass createTexture:self.renderPicture];
    
    render_pic = xhey_init_picture_textureId(render_pic_texture_id, 500, 500, 0);
    
    basic = xhey_init_basic_filter(context);
    
    output = xhey_init_picture_output(context, 500, 500, 0);
    
    lut = xhey_init_lookup_filter(context);
    lookup_textureId = [XLHelpClass setupTexture:[UIImage imageNamed:@"b_street_food"]];
    pic = xhey_init_picture_textureId(lookup_textureId, 512, 512, 0);
    
    xhey_picture_graph(g, render_pic, basic, pic, lut, 0, 0, output);
    
    return self;
}


- (void) renderPictureWithLut:(NSString*)lut
{
    [GPUImageContext useImageProcessingContext];
    [self texImageTexture:lut];
    
    xhey_graph_forward(g);
    
    textureId = xhey_picture_output_get_texture_id(output);
    
    [_glView renderTextureId:textureId];
}



- (instancetype)initWithInput:(CameraEntry*) cameraEntry
                   renderView:(OpenGLView*)glView
                       writer:(MovieWriter*)movieWriter
                      context:(EAGLContext*)context
{
    self = [super init];
    if (!self) {
        return nil;
    }
    
    isPhoto = false;
    
    currentContext = [[GPUImageContext sharedImageProcessingContext] context];
    ctxt = (Context*)malloc(sizeof(Context));
    ctxt->self = self;
    
    self.cameraEntry = cameraEntry;
    [self.cameraEntry setVideoOutputDelegate:self];
    self.glView = glView;
    self.movieWriter = movieWriter;
    
    return self;
}
- (void) startCapture
{
    [self.cameraEntry startCapture];
}

- (void) stopCapture
{
    [self.cameraEntry stopCapture];
}

- (void) changeLookup:(NSString*) path
{
    lutUpdate = YES;
    lut_path = path;
    
    if (isPhoto) {
        [self renderPictureWithLut:path];
    }
}

- (void) startRecordWithWaterInfo:( WaterViewInfo *  ) waterInfo destinationURL:(NSURL *)url
{
    [_movieWriter start];
    
}

- (void) stopRecordWithCompletion:(void (^)(NSError * _Nonnull))handler {
    [_movieWriter stop];
}


- (void) texImageTexture:(NSString*)path{
    
    if (!path) {
        return;
    }
    
    UIImage* image = [[UIImage alloc] initWithContentsOfFile:path];
    CGImage* newImageSource = [image CGImage];
    int width = (int)CGImageGetWidth(newImageSource);
    int height = (int)CGImageGetHeight(newImageSource);
    
    GLubyte *imageData = (GLubyte*)calloc(1, width*height*4);
    CGColorSpaceRef genericRGBColorspace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext = CGBitmapContextCreate(imageData, width, height, 8, width*4, genericRGBColorspace, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext, CGRectMake(0, 0, width, height), newImageSource);
    
    xhey_picture_update(pic, imageData, width, height);
    
    
    CGContextRelease(imageContext);
    CGColorSpaceRelease(genericRGBColorspace);
    free(imageData);
    newImageSource = nil;
}
                                 
                                 
- (void)switchCamera {
    if (_cameraEntry.location == AVCaptureDevicePositionFront) {
        _cameraEntry.location = AVCaptureDevicePositionBack;
    }else{
        _cameraEntry.location = AVCaptureDevicePositionFront;
    }
}

- (void)captureOutput:(AVCaptureOutput *)_output didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer fromConnection:(AVCaptureConnection *)connection{
    
    CMTime frameTime = CMSampleBufferGetPresentationTimeStamp(sampleBuffer);
    
    CVPixelBufferRef cameraFrame = CMSampleBufferGetImageBuffer(sampleBuffer);
    CVPixelBufferLockBaseAddress(cameraFrame, 0);
    
    int width = (int)round(CVPixelBufferGetWidth(cameraFrame));
    int height = (int)round(CVPixelBufferGetHeight(cameraFrame));
    
    int _width = aw_stride(width);
    
    void* y_frame = (void*)CVPixelBufferGetBaseAddressOfPlane(cameraFrame, 0);
    void* uv_frame = (void*)CVPixelBufferGetBaseAddressOfPlane(cameraFrame, 1);
    
    if (isFirst == FALSE) {
        isFirst = TRUE;
        
        [EAGLContext setCurrentContext:currentContext];
        
        glGenTextures(1, &y_textureId);
        glBindTexture(GL_TEXTURE_2D, y_textureId);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
        
        glTexImage2D(GL_TEXTURE_2D, 0, GL_LUMINANCE, _width, height, 0, GL_LUMINANCE, GL_UNSIGNED_BYTE, y_frame);
        glBindTexture(GL_TEXTURE_2D, 0);
        
        
        glGenTextures(1, &uv_textureId);
        glBindTexture(GL_TEXTURE_2D, uv_textureId);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
        
        glTexImage2D(GL_TEXTURE_2D, 0, GL_LUMINANCE_ALPHA, _width / 2 , height / 2 , 0, GL_LUMINANCE_ALPHA, GL_UNSIGNED_BYTE, uv_frame);
        glBindTexture(GL_TEXTURE_2D, 0);
        
        
        g = xhey_init_graph();
        context = init_context();
        cam = xhey_init_camera(context, width, height, 0);
        camera_update_luminance(cam, y_textureId);
        camera_update_chrominance(cam, uv_textureId);
        float mat[9] = {1.0f, 1.0f, 1.0f,
            0.0f, -0.343f, 1.765f,
            1.4f, -0.711f, 0.0f};
        
        camera_update_matrix(cam, mat);
        basic = xhey_init_basic_filter(context);
        xhey_update_basic_hook(basic, print1, (void*)ctxt);
        lut = xhey_init_lookup_filter(context);
        lookup_textureId = [XLHelpClass setupTexture:[UIImage imageNamed:@"b_street_food"]];
        pic = xhey_init_picture_textureId(lookup_textureId, 512, 512, 0);
        
        output = xhey_init_picture_output(context, width, height, 3);
        xhey_update_picture_output_hook(output, print_test1,(void*)ctxt);
        xhey_picture_graph(g, cam, basic, pic, lut, 0, 0, output);
        
        
    }else{
        [EAGLContext setCurrentContext:currentContext];
        
        glBindTexture(GL_TEXTURE_2D, y_textureId);
        glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, _width, height, GL_LUMINANCE, GL_UNSIGNED_BYTE, y_frame);
        glBindTexture(GL_TEXTURE_2D, 0);
        
        
        glBindTexture(GL_TEXTURE_2D, uv_textureId);
        glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, _width / 2  , height / 2 , GL_LUMINANCE_ALPHA, GL_UNSIGNED_BYTE, uv_frame);
        glBindTexture(GL_TEXTURE_2D, 0);
        
        
        camera_update_size(cam, _width, height);
        
        if (lutUpdate) {
            lutUpdate = NO;
            [self texImageTexture:lut_path];
        }
        
    }
    
    
    xhey_graph_forward(g);
    textureId = xhey_picture_output_get_texture_id(output);
    
    
    
    
    [_glView renderTextureId:textureId];
    
    
    {
        int _width = width;
        int _height = height;
        CVPixelBufferRef pxbuffer = NULL;
        
        NSDictionary *options = [NSDictionary dictionaryWithObjectsAndKeys:
                                 [NSNumber numberWithBool:YES], kCVPixelBufferCGImageCompatibilityKey,
                                 [NSNumber numberWithBool:YES], kCVPixelBufferCGBitmapContextCompatibilityKey,
                                 nil];
        
        CVReturn status = CVPixelBufferCreate(kCFAllocatorDefault, _width,
                                              _height, kCVPixelFormatType_32BGRA, (__bridge CFDictionaryRef) options,
                                              &pxbuffer);
        
        CVPixelBufferLockBaseAddress(pxbuffer, 0);
        void *pxdata = CVPixelBufferGetBaseAddress(pxbuffer);
        
        glReadPixels(0, 0, _width, _height, GL_BGRA, GL_UNSIGNED_BYTE, pxdata);
        
        
        CVPixelBufferUnlockBaseAddress(pxbuffer, 0);
        CVPixelBufferRelease(pxbuffer);
    }
    
//
    if (_movieWriter) {
        int _width = height;
        int _height = width;
        [_movieWriter readAndPut:_width width:height frameTime:frameTime];
    }
    
    [EAGLContext setCurrentContext:nil];
    
    CVPixelBufferUnlockBaseAddress(cameraFrame, 0);
    
}

- (void)capturePhotoWithWater:(WaterViewInfo *)waterInfo previewImgCallBack:(void (^)(UIImage * _Nonnull, NSError * _Nonnull))previewImgCallBack originalImgCallBack:(void (^)(UIImage * _Nonnull, NSError * _Nonnull))originalImgCallBack processedImgCallBack:(void (^)(UIImage * _Nonnull, NSError * _Nonnull))processedImgCallBack{
    
    [_cameraEntry takePhotoWithCompletionHandle:^(CVPixelBufferRef imagePixelBuffer) {
        
        CVPixelBufferLockBaseAddress(imagePixelBuffer, 0);
        
        int bufferWidth = (int)CVPixelBufferGetWidth(imagePixelBuffer);
        int bufferHeight = (int)CVPixelBufferGetHeight(imagePixelBuffer);
        
        OSType pixelFormat = CVPixelBufferGetPixelFormatType(imagePixelBuffer);
        
        
        if (pixelFormat == kCVPixelFormatType_32BGRA) {
            NSLog(@"ddddd");
        }
        
        if (pixelFormat == kCVPixelFormatType_420YpCbCr8BiPlanarFullRange || pixelFormat == kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange) {
            NSLog(@"wwwww");
        }
        
        CVPixelBufferUnlockBaseAddress(imagePixelBuffer, 0);
        
    }];
    
}

@end
