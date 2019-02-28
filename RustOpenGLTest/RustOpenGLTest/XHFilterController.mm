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
#import "XHImageContext.h"
struct Context{
    XHFilterController* self;
};

@implementation WaterViewInfo
@end

@interface XHFilterController()<CameraEntryDelegate>
{
    EAGLContext* currentContext;
    
    long g;
    
    
    long render_pic;
    long cam;
    
    long lut;
    long pic;
    
    long surface;
    long basic;
    long basic_normal;
    long output;
    long normal_output;
    
    long context;
    
    long context_watermark_ptr;
    long watermark_graph;
    long watermark_picture_ptr;
    long watermark_basic_ptr;
    long watermark_watermark_ptr;
    long watermark_output_ptr;
    
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
    
    NSLock* lock;
    XHFilterControllerMode _mode;
    
    GLuint image_texid;

}
@property (nonatomic, strong) CameraEntry* cameraEntry;
@property (nonatomic, strong) OpenGLView* glView;
@property (nonatomic, strong) MovieWriter* movieWriter;
@property (nonatomic, strong) UIImage* renderPicture;
@end
@implementation XHFilterController
#define aw_stride(wid) ((wid % 16 != 0) ? ((wid) + 16 - (wid) % 16): (wid))
void print1(void* context){

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
    [XHImageContext useImageProcessingContext];
    [self texImageTexture:lut];
    
    xhey_graph_forward(g);
    
    textureId = xhey_picture_output_get_texture_id(output);
    
    [_glView renderTextureId:textureId];
}

- (void)clear{
    if (!isPhoto) {
        
        release_output(output);
        output = 0;
        
        release_picture(pic);
        pic = 0;
        
        glDeleteTextures(1, &lookup_textureId);
        lookup_textureId = 0;
        
        release_lookup_filter(lut);
        lut = 0;
        
        
        glDeleteTextures(1, &y_textureId);
        y_textureId = 0;
        
        glDeleteTextures(1, &uv_textureId);
        uv_textureId = 0;
        
        release_basic_filter(basic);
        basic = 0;
        
        release_camera(cam);
        cam = 0;
        
        release_context(context);
        context = 0;
        
        release_graph(g);
        g = 0;
    }
}

- (instancetype)initWithInput:(CameraEntry*) cameraEntry
                   renderView:(OpenGLView*)glView
{
    self = [super init];
    if (!self) {
        return nil;
    }
    
    isPhoto = false;
    
    lock = [[NSLock alloc] init];
    
    currentContext = [[XHImageContext sharedImageProcessingContext] context];
    ctxt = (Context*)malloc(sizeof(Context));
    ctxt->self = self;
    
    self.cameraEntry = cameraEntry;
    _cameraEntry.delegate = self;
//    [self.cameraEntry setVideoOutputDelegate:self];
    self.glView = glView;
    
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
- (void)changeFilter:(XHFilterControllerMode)mode {
    _mode = mode;
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
    
    CGSize frameSize = CGSizeMake(720, 1280);
    NSString* pathToMovie = [NSHomeDirectory() stringByAppendingPathComponent:@"Documents/Movie.m4v"];
    unlink([pathToMovie UTF8String]); // If a file already exists, AVAssetWriter won't let you record new frames, so delete the old movie
    NSURL* movieURL = [NSURL fileURLWithPath:pathToMovie];
    
    
    self.movieWriter = [[MovieWriter alloc] initWithFrameSize:frameSize movieURL:movieURL];

    [_movieWriter start];
    
}

- (void) stopRecordWithCompletion:(void (^)(NSError * _Nonnull))handler {
    [_movieWriter stop];
    _movieWriter = nil;
    [self clearWriterGraph];
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
//    [self clear];
//    isFirst = FALSE;
    
    textureId = 0;
    if (_cameraEntry.location == AVCaptureDevicePositionFront) {
        _cameraEntry.location = AVCaptureDevicePositionBack;
    }else{
        _cameraEntry.location = AVCaptureDevicePositionFront;
    }
}

- (void)processAudioBuffer:(CMSampleBufferRef)audioBuffer {
    
}
- (void)processVideoBuffer:(CMSampleBufferRef)sampleBuffer{
    
    CMTime frameTime = CMSampleBufferGetPresentationTimeStamp(sampleBuffer);
    
    CVPixelBufferRef cameraFrame = CMSampleBufferGetImageBuffer(sampleBuffer);
    CVPixelBufferLockBaseAddress(cameraFrame, 0);
    
    int width = (int)round(CVPixelBufferGetWidth(cameraFrame));
    int height = (int)round(CVPixelBufferGetHeight(cameraFrame));
    
//    int _width = aw_stride(width);
    int _width = width;
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
        cam = xhey_init_camera(context, _width, height, 0);
        camera_update_luminance(cam, y_textureId);
        camera_update_chrominance(cam, uv_textureId);
        float mat[9] = {1.0f, 1.0f, 1.0f,
            0.0f, -0.343f, 1.765f,
            1.4f, -0.711f, 0.0f};
        
        camera_update_matrix(cam, mat);
        basic = xhey_init_basic_filter(context);
        basic_normal = xhey_init_basic_filter(context);
        
        xhey_update_basic_hook(basic, print1, (void*)ctxt);
        lut = xhey_init_lookup_filter(context);
        lookup_textureId = [XLHelpClass setupTexture:[UIImage imageNamed:@"b_street_food"]];
        pic = xhey_init_picture_textureId(lookup_textureId, 512, 512, 0);
        
        output = xhey_init_picture_output(context, _width, height, 3);
        normal_output = xhey_init_picture_output(context, _width, height, 3);
        xhey_update_picture_output_hook(output, print_test1,(void*)ctxt);
        xhey_camera_graph(g, cam, basic,0, pic, lut, 0, 0, output,normal_output);
        
    }else{
        [EAGLContext setCurrentContext:currentContext];
        
        glBindTexture(GL_TEXTURE_2D, y_textureId);
//        glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, _width, height, GL_LUMINANCE, GL_UNSIGNED_BYTE, y_frame);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_LUMINANCE, _width, height, 0, GL_LUMINANCE, GL_UNSIGNED_BYTE, y_frame);
        glBindTexture(GL_TEXTURE_2D, 0);
        
        
        glBindTexture(GL_TEXTURE_2D, uv_textureId);
//        glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, _width / 2  , height / 2 , GL_LUMINANCE_ALPHA, GL_UNSIGNED_BYTE, uv_frame);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_LUMINANCE_ALPHA, _width / 2 , height / 2 , 0, GL_LUMINANCE_ALPHA, GL_UNSIGNED_BYTE, uv_frame);
        glBindTexture(GL_TEXTURE_2D, 0);
        
        
        camera_update_size(cam, _width, height);
        xhey_update_output_size(output, _width, height);
        
        if (lutUpdate) {
            lutUpdate = NO;
            [self texImageTexture:lut_path];
        }
        
    }
    
    
    xhey_graph_forward(g);
    
    if (_mode == XHFilterControllerModeNormal) {
        textureId = xhey_picture_output_get_texture_id(normal_output);
    }else{
        textureId = xhey_picture_output_get_texture_id(output);
    }
    
    
    [_glView renderTextureId:textureId];

    if (_movieWriter) {
        [EAGLContext setCurrentContext:currentContext];

        if (_movieWriter.isReady == FALSE) {
            _movieWriter.isReady = TRUE;

            watermark_graph = xhey_init_graph();
            context_watermark_ptr = init_context();
            watermark_picture_ptr = xhey_init_picture_textureId(textureId, width, height, 0);
            watermark_basic_ptr = xhey_init_basic_filter(context_watermark_ptr);
            watermark_watermark_ptr = xhey_init_watermark(context_watermark_ptr);

            UIImage* image = [UIImage imageNamed:@"aaa"];
            image_texid = [XLHelpClass createTexture:image];

            xhey_watermark_update(watermark_watermark_ptr, image_texid, -1.0, 0.0, 0.5, 0.5,0);

            watermark_output_ptr = xhey_init_picture_output(context_watermark_ptr, width, height, 0);

            xhey_camera_watermark_graph(watermark_graph, watermark_picture_ptr, watermark_basic_ptr, watermark_watermark_ptr, watermark_output_ptr);


        }
        xhey_graph_forward(watermark_graph);

        int _width = height;
        int _height = width;
        [_movieWriter readAndPutWithWidth:_width height:height frameTime:frameTime];
//        [_movieWriter readAndPut:_width width:height frameTime:frameTime];
    }
    
    [EAGLContext setCurrentContext:nil];
    
    CVPixelBufferUnlockBaseAddress(cameraFrame, 0);
    
}

- (void) clearWriterGraph
{
    release_output(watermark_output_ptr);
    release_water_mark_filter(watermark_watermark_ptr);
    release_basic_filter(watermark_basic_ptr);
    release_picture(watermark_picture_ptr);
    release_graph(watermark_graph);
    release_context(context_watermark_ptr);
    glDeleteTextures(1, &image_texid);
}

- (void)capturePhotoWithWater:(WaterViewInfo *)waterInfo previewImgCallBack:(void (^)(UIImage * _Nonnull, NSError * _Nonnull))previewImgCallBack originalImgCallBack:(void (^)(UIImage * _Nonnull, NSError * _Nonnull))originalImgCallBack processedImgCallBack:(void (^)(UIImage * _Nonnull, NSError * _Nonnull))processedImgCallBack{
    
    [_cameraEntry takePhotoWithCompletionHandle:^(CVPixelBufferRef imagePixelBuffer, NSError *) {
        
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
