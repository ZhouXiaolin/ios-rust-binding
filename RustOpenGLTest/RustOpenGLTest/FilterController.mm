//
//  FilterController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2019/1/22.
//  Copyright © 2019 Solaren. All rights reserved.
//

#import "FilterController.h"
#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "gpuimage.h"
#import "XLHelpClass.h"
#import "CameraEntry.h"
#import "OpenGLView.h"
#import "MovieWriter.h"
struct Context{
    FilterController* self;
};

@implementation WaterViewInfo
@end

@interface FilterController()
{
    EAGLContext* currentContext;
    
    long g;
    long lut;
    long pic;
    long cam;
    long surface;
    long basic;
    long output;
    long context;
    
    
    GLuint y_textureId;
    GLuint uv_textureId;
    GLuint textureId;
    GLuint lookup_textureId;
    
    BOOL isFirst;
    
    BOOL update;
    
    Context* ctxt;

    NSString* lut_path;
    BOOL lutUpdate;


}
@property (nonatomic, strong) CameraEntry* cameraEntry;
@property (nonatomic, strong) OpenGLView* glView;
@property (nonatomic, strong) MovieWriter* movieWriter;
@end
@implementation FilterController
#define aw_stride(wid) ((wid % 16 != 0) ? ((wid) + 16 - (wid) % 16): (wid))
void print1(void* context){
    NSLog(@"TTTTTTTTTTTTT --------");
}

void print_test1(void* context){
    Context* ctxt = (Context*)context;
    
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
    
    currentContext = context;
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
}

- (void) startRecordWithWaterInfo:( WaterViewInfo *  ) waterInfo destinationURL:(NSURL *)url
{
    [_movieWriter start];
    
}

- (void) stopRecordWithCompletion:(void (^)(NSError * _Nonnull))handler {
    [_movieWriter stop];
}

- (void) texImageTexture:(NSString*)path{
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
        
        
        if (lutUpdate) {
            lutUpdate = NO;
            [self texImageTexture:lut_path];
        }
        
    }
    
    
    xhey_graph_forward(g);
    textureId = xhey_picture_output_get_texture_id(output);
    
    
    
    [_glView renderTextureId:textureId];
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
        
        
        
    }];
    
}
@end
