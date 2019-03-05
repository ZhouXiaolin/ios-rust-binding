//
//  CameraEntryObjC.m
//  XCamera
//
//  Created by 周晓林 on 2018/7/30.
//  Copyright © 2018年 xhey. All rights reserved.
//

#import "CameraEntry.h"
#import "XLHelpClass.h"
#import "XHImageContext.h"
typedef void(^CompleteHandle)(UIImage*);


void stillImageDataReleaseCallback(void *releaseRefCon, const void *baseAddress)
{
    free((void *)baseAddress);
}

void GPUImageCreateResizedSampleBuffer(CVPixelBufferRef cameraFrame, CGSize finalSize, CVPixelBufferRef *pixelbuffer)
{
    // CVPixelBufferCreateWithPlanarBytes for YUV input
    
    CGSize originalSize = CGSizeMake(CVPixelBufferGetWidth(cameraFrame), CVPixelBufferGetHeight(cameraFrame));
    
    CVPixelBufferLockBaseAddress(cameraFrame, 0);
    GLubyte *sourceImageBytes =  (GLubyte*)CVPixelBufferGetBaseAddress(cameraFrame);
    CGDataProviderRef dataProvider = CGDataProviderCreateWithData(NULL, sourceImageBytes, CVPixelBufferGetBytesPerRow(cameraFrame) * originalSize.height, NULL);
    CGColorSpaceRef genericRGBColorspace = CGColorSpaceCreateDeviceRGB();
    CGImageRef cgImageFromBytes = CGImageCreate((int)originalSize.width, (int)originalSize.height, 8, 32, CVPixelBufferGetBytesPerRow(cameraFrame), genericRGBColorspace, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst, dataProvider, NULL, NO, kCGRenderingIntentDefault);
    
    GLubyte *imageData = (GLubyte *) calloc(1, (int)finalSize.width * (int)finalSize.height * 4);
    
    CGContextRef imageContext = CGBitmapContextCreate(imageData, (int)finalSize.width, (int)finalSize.height, 8, (int)finalSize.width * 4, genericRGBColorspace,  kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext, CGRectMake(0.0, 0.0, finalSize.width, finalSize.height), cgImageFromBytes);
    CGImageRelease(cgImageFromBytes);
    CGContextRelease(imageContext);
    CGColorSpaceRelease(genericRGBColorspace);
    CGDataProviderRelease(dataProvider);
    
    CVPixelBufferCreateWithBytes(kCFAllocatorDefault, finalSize.width, finalSize.height, kCVPixelFormatType_32BGRA, imageData, finalSize.width * 4, stillImageDataReleaseCallback, NULL, NULL, pixelbuffer);
    
}


@interface CameraEntry () <AVCapturePhotoCaptureDelegate,AVCaptureVideoDataOutputSampleBufferDelegate, AVCaptureAudioDataOutputSampleBufferDelegate,AVCapturePhotoCaptureDelegate>
{
    
    
    float preScale;
    AVCaptureSession *captureSession;
    AVCaptureDeviceInput *videoInput;
    AVCaptureVideoDataOutput *videoOutput;
    dispatch_queue_t cameraQueue,audioQueue;
    
    
    AVCaptureDevice *microphone;
    
#ifdef USE_STILLIMAGE
    
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
    AVCaptureStillImageOutput* imageOutput;
#pragma clang diagnostic pop
    
#else
    CompleteHandle completeImageHandle;
    AVCapturePhotoOutput* imageOutput;
#endif
    
    AVCaptureDeviceInput* audioInput;
    AVCaptureAudioDataOutput* audioOutput;
    
    dispatch_semaphore_t frameRenderingSemaphore;
    BOOL isFullYUVRange;
    
    id<AVCaptureVideoDataOutputSampleBufferDelegate> _cameraInput;
    id<AVCaptureAudioDataOutputSampleBufferDelegate> _audioInput;
    
}
@property (nonatomic, assign) CFAbsoluteTime lastCheckTime;
@property (nonatomic, assign) int framesSinceLastCheck;
@property (nonatomic, assign) AVCaptureVideoStabilizationMode videoStabilizationMode;
@end

@implementation CameraEntry

- (AVCaptureVideoStabilizationMode)videoStabilizationMode{
    NSInteger stabilizationMode = [[NSUserDefaults standardUserDefaults] integerForKey:@"video_stabilization_default"];
    AVCaptureVideoStabilizationMode mode = AVCaptureVideoStabilizationModeOff;
    if (stabilizationMode == 1) {
        mode = AVCaptureVideoStabilizationModeOff;
    }
    if (stabilizationMode == 2) {
        mode = AVCaptureVideoStabilizationModeStandard;
    }
    if (stabilizationMode != 1 && stabilizationMode != 2) {
        stabilizationMode = 1;
        [[NSUserDefaults standardUserDefaults] setInteger:stabilizationMode forKey:@"video_stabilization_default"];
    }
    return mode;
}

- (void) setVideoOutputDelegate:(id<AVCaptureVideoDataOutputSampleBufferDelegate>) delegate
{
    _cameraInput = delegate;
}


- (void) setCameraAdjustParameter {
    NSError* error;
    [_inputCamera lockForConfiguration:&error];
    
    [_inputCamera setSubjectAreaChangeMonitoringEnabled:FALSE]; // 开启范围监测
    
    if ([_inputCamera isAutoFocusRangeRestrictionSupported]) { //自动对焦区域限制 无
        [_inputCamera setAutoFocusRangeRestriction:(AVCaptureAutoFocusRangeRestrictionNone)];
    }
    if ([_inputCamera isFocusModeSupported:(AVCaptureFocusModeContinuousAutoFocus)]) { // 对焦模式
        [_inputCamera setFocusMode:(AVCaptureFocusModeContinuousAutoFocus)];
        if ([_inputCamera isSmoothAutoFocusSupported]) {// 平滑对焦
            [_inputCamera setSmoothAutoFocusEnabled:TRUE];
        }
    }
    
    if ([_inputCamera isExposureModeSupported:(AVCaptureExposureModeContinuousAutoExposure)]) { // 曝光模式
        [_inputCamera setExposureMode:(AVCaptureExposureModeContinuousAutoExposure)];
    }
    
    if ([_inputCamera isWhiteBalanceModeSupported:(AVCaptureWhiteBalanceModeContinuousAutoWhiteBalance)]) { // 白平衡模式
        [_inputCamera setWhiteBalanceMode:(AVCaptureWhiteBalanceModeContinuousAutoWhiteBalance)];
    }
    
    if ([_inputCamera isLowLightBoostSupported]) { // 弱光下自动提升亮度
        [_inputCamera setAutomaticallyEnablesLowLightBoostWhenAvailable:TRUE];
    }
    
    [_inputCamera unlockForConfiguration];
}



- (instancetype)initWithSessionPreset:(AVCaptureSessionPreset) sessionPreset
                             location:(AVCaptureDevicePosition) location
                      cameraEntryMode:(CameraEntryMode)cameraEntryMode
                         captureAsYUV:(BOOL)captureAsYUV
{
    if (!(self = [super init])) {
        return nil;
    }
    
    
    
    self.framesSinceLastCheck = 0;
    self.lastCheckTime = CFAbsoluteTimeGetCurrent();
    
    preScale = 0.0;
    
    _cameraMode = cameraEntryMode;
    
    cameraQueue = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH,0);
    audioQueue = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_LOW, 0);
    
    self.location = location;
    
    frameRenderingSemaphore = dispatch_semaphore_create(1);
    
    
    captureSession = [[AVCaptureSession alloc] init];
    [captureSession beginConfiguration];
    
    _inputCamera = [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeBuiltInWideAngleCamera, AVCaptureDeviceTypeBuiltInTelephotoCamera] mediaType:AVMediaTypeVideo position:location].devices.firstObject;
    
    if (!_inputCamera) {
        return nil;
    }
    
    
    
    [self setCameraAdjustParameter];
    
    NSError* error;
    videoInput = [[AVCaptureDeviceInput alloc] initWithDevice:_inputCamera error:&error];
    if ([captureSession canAddInput:videoInput]) {
        [captureSession addInput:videoInput];
    }
    
    videoOutput = [[AVCaptureVideoDataOutput alloc] init];
    [videoOutput setAlwaysDiscardsLateVideoFrames:NO];
    [videoOutput setSampleBufferDelegate:self queue:cameraQueue];
    
    
    
    if (captureAsYUV) {
        BOOL supportsFullYUVRange = NO;
        NSArray* supportedPixelFormats = videoOutput.availableVideoCVPixelFormatTypes;
        for (NSNumber *currentPixelFormat in supportedPixelFormats) {
            if ([currentPixelFormat intValue] == kCVPixelFormatType_420YpCbCr8BiPlanarFullRange) {
                supportsFullYUVRange = YES;
            }
        }
        if (supportsFullYUVRange)
        {
            self.pixelFormatType =  kCVPixelFormatType_420YpCbCr8BiPlanarFullRange;
            
            [videoOutput setVideoSettings:[NSDictionary dictionaryWithObject:[NSNumber numberWithInt:kCVPixelFormatType_420YpCbCr8BiPlanarFullRange] forKey:(id)kCVPixelBufferPixelFormatTypeKey]];
            isFullYUVRange = YES;
        }
        else
        {
            self.pixelFormatType = kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange;
            [videoOutput setVideoSettings:[NSDictionary dictionaryWithObject:[NSNumber numberWithInt:kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange] forKey:(id)kCVPixelBufferPixelFormatTypeKey]];
            isFullYUVRange = NO;
        }
        
    }else{
        self.pixelFormatType = kCVPixelFormatType_32BGRA;
        [videoOutput setVideoSettings:[NSDictionary dictionaryWithObject:[NSNumber numberWithInt:kCVPixelFormatType_32BGRA] forKey:(id)kCVPixelBufferPixelFormatTypeKey]];
    }
    
    
    if ([captureSession canAddOutput:videoOutput]) {
        [captureSession addOutput:videoOutput];
    }else{
        NSLog(@"Couldn't add video output");
        return nil;
    }
    
#ifdef USE_STILLIMAGE
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
    imageOutput = [[AVCaptureStillImageOutput alloc] init];
    //    [imageOutput setOutputSettings:[NSDictionary dictionaryWithObject:[NSNumber numberWithInt:kCVPixelFormatType_420YpCbCr8BiPlanarFullRange] forKey:(id)kCVPixelBufferPixelFormatTypeKey]];
    
    [imageOutput setOutputSettings:[NSDictionary dictionaryWithObject:[NSNumber numberWithInt:kCVPixelFormatType_32BGRA] forKey:(id)kCVPixelBufferPixelFormatTypeKey]];
    
    
    
    [imageOutput setHighResolutionStillImageOutputEnabled:YES];
    
    if ([captureSession canAddOutput:imageOutput]) {
        [captureSession addOutput:imageOutput];
    }
#pragma clang diagnostic pop
#else
    imageOutput = [[AVCapturePhotoOutput alloc] init];
    [imageOutput setHighResolutionCaptureEnabled:TRUE];
    if ([captureSession canAddOutput:imageOutput]) {
        [captureSession addOutput:imageOutput];
    }
#endif
    
    if([captureSession canSetSessionPreset:sessionPreset]){
        [captureSession setSessionPreset:sessionPreset];
    }
    
    
    [captureSession commitConfiguration];
    
    
    
    return self;
}

- (void)dealloc{
    [self stopCapture];
    [videoOutput setSampleBufferDelegate:nil queue:nil];
    [audioOutput setSampleBufferDelegate:nil queue:nil];
    
    
    [[NSNotificationCenter defaultCenter] removeObserver:self name:UIApplicationDidBecomeActiveNotification object:nil];
    [[NSNotificationCenter defaultCenter] removeObserver:self name:UIApplicationWillEnterForegroundNotification object:nil];
    [[NSNotificationCenter defaultCenter] removeObserver:self name:UIApplicationDidEnterBackgroundNotification object:nil];
}

- (void)setLocation:(AVCaptureDevicePosition)location{
    _location = location;
    [self switchCamera];
}

- (void)setIsBackMirrord:(BOOL)isBackMirrord{
    _isBackMirrord = isBackMirrord;
    [self updateMirror];
}
- (void)setIsFrontMirrord:(BOOL)isFrontMirrord{
    _isFrontMirrord = isFrontMirrord;
    [self updateMirror];
}

- (void) startCapture {
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        if (![self->captureSession isRunning]) {
            [self->captureSession startRunning];
        }
    });
}

- (void) stopCapture {
    if ([captureSession isRunning]) {
        [captureSession stopRunning];
    }
}
- (BOOL)isRunning{
    if ([captureSession isRunning]) {
        return true;
    }else{
        return false;
    }
}

- (BOOL) canSwitchCameras
{
    return [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeBuiltInWideAngleCamera,AVCaptureDeviceTypeBuiltInTelephotoCamera] mediaType:AVMediaTypeVideo position:AVCaptureDevicePositionUnspecified].devices.count > 1;
}
- (void) addAudioInputsAndOutputs
{
    if (audioOutput != nil) {
        return;
    }
    
    [captureSession beginConfiguration];
    
    microphone = [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeBuiltInMicrophone] mediaType:AVMediaTypeAudio position:AVCaptureDevicePositionUnspecified].devices.firstObject;
    NSError* error;
    audioInput = [AVCaptureDeviceInput deviceInputWithDevice:microphone error:&error];
    if ([captureSession canAddInput:audioInput]) {
        [captureSession addInput:audioInput];
    }
    audioOutput = [[AVCaptureAudioDataOutput alloc] init];
    if ([captureSession canAddOutput:audioOutput]) {
        [captureSession addOutput:audioOutput];
    }
    [audioOutput setSampleBufferDelegate:self queue:audioQueue];
    
    [captureSession commitConfiguration];
}

- (void) addAudioInputsAndOutputsWithDelegate:(id<AVCaptureAudioDataOutputSampleBufferDelegate>) delegate// callbackQueue:(dispatch_queue_t) callbackQueue
{
    if (audioOutput != nil) {
        return;
    }
    
    _audioInput = delegate;
    
    
    [captureSession beginConfiguration];
    
    microphone = [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeBuiltInMicrophone] mediaType:AVMediaTypeAudio position:AVCaptureDevicePositionUnspecified].devices.firstObject;
    NSError* error;
    audioInput = [AVCaptureDeviceInput deviceInputWithDevice:microphone error:&error];
    if ([captureSession canAddInput:audioInput]) {
        [captureSession addInput:audioInput];
    }
    audioOutput = [[AVCaptureAudioDataOutput alloc] init];
    if ([captureSession canAddOutput:audioOutput]) {
        [captureSession addOutput:audioOutput];
    }
    [audioOutput setSampleBufferDelegate:self queue:audioQueue];
    
    [captureSession commitConfiguration];
    
}



- (void) removeAudioInputsAndOutputs
{
    if (audioOutput == nil) {
        return;
    }
    
    [captureSession beginConfiguration];
    [audioOutput setSampleBufferDelegate:nil queue:nil];
    [captureSession removeInput:audioInput];
    [captureSession removeOutput:audioOutput];
    audioInput = nil;
    audioOutput = nil;
    microphone = nil;
    [captureSession commitConfiguration];
}

- (void) zoomWithScale:(CGFloat) scale
{
    NSError* error;
    [_inputCamera lockForConfiguration:&error];
    
    float videoZoomFactor = [_inputCamera videoZoomFactor] + (scale - preScale) * 15.0;
    videoZoomFactor = videoZoomFactor <= 1.0 ? 1.0 : videoZoomFactor;
    float maxFactor;
    if ([_inputCamera activeFormat].videoMaxZoomFactor >= 3.0) {
        maxFactor = 3.0;
    }else{
        maxFactor = [_inputCamera activeFormat].videoMaxZoomFactor;
    }
    
    videoZoomFactor = videoZoomFactor >= maxFactor ? maxFactor : videoZoomFactor;
    
    [_inputCamera setVideoZoomFactor:videoZoomFactor];
    
    preScale = scale;
    
    [_inputCamera unlockForConfiguration];
}

- (void) zoomBegin:(CGFloat) scale
{
    preScale = scale;
}

- (void) zoomEnd:(CGFloat) scale
{
    preScale = scale;
}

- (void) cancelZoom
{
    NSError* error;
    [_inputCamera lockForConfiguration:&error];
    [_inputCamera cancelVideoZoomRamp];
    [_inputCamera unlockForConfiguration];
}

- (void) focusAtPoint:(CGPoint) point isSmooth:(BOOL) isSmooth focusMode:(AVCaptureFocusMode) mode
{
    NSError* error;
    [_inputCamera lockForConfiguration:&error];
    
    if ([_inputCamera isFocusPointOfInterestSupported]) {
        [_inputCamera setFocusPointOfInterest:point];
        [_inputCamera setFocusMode:AVCaptureFocusModeContinuousAutoFocus];
    }
    [_inputCamera unlockForConfiguration];
    
}

- (void) exposureAtPoint:(CGPoint) point exposureMode:(AVCaptureExposureMode) mode
{
    NSError* error;
    [_inputCamera lockForConfiguration:&error];
    
    if ([_inputCamera isExposurePointOfInterestSupported]) {
        [_inputCamera setExposurePointOfInterest:point];
        [_inputCamera setExposureMode:AVCaptureExposureModeContinuousAutoExposure];
    }
    [_inputCamera unlockForConfiguration];
}

- (void) setExposure:(float) bias
{
    NSError* error = nil ;
    [_inputCamera lockForConfiguration:&error];
    float min = _inputCamera.minExposureTargetBias;
    float max = _inputCamera.maxExposureTargetBias;
    [_inputCamera setExposureTargetBias:min + (max - min) * bias completionHandler:nil];
    [_inputCamera unlockForConfiguration];
}


- (void) setStabilitizationMode:(AVCaptureVideoStabilizationMode) mode
{
    AVCaptureConnection* connection = [videoOutput connectionWithMediaType:AVMediaTypeVideo];
    if ([connection isVideoStabilizationSupported]) {
        if ([connection preferredVideoStabilizationMode] == _videoStabilizationMode) {
            return;
        }
        [connection setPreferredVideoStabilizationMode:_videoStabilizationMode];
    }
}

- (void) switchCamera
{
    if (![self canSwitchCameras]) {
        return;
    }
    
    if (_inputCamera.position == _location) {
        return;
    }
    
    
    
    [captureSession beginConfiguration];
    AVCaptureDevice* oldDevice = _inputCamera;
    AVCaptureDeviceInput* oldDeviceInput = videoInput;
    
    [captureSession removeInput:videoInput];
    
    AVCaptureSessionPreset videoPresent = (_location == AVCaptureDevicePositionBack ? ([XLHelpClass isLowerThaniPhone6] ? AVCaptureSessionPreset1280x720 : AVCaptureSessionPreset1920x1080) : AVCaptureSessionPreset1280x720);
    AVCaptureSessionPreset imagePresent = _cameraMode == CameraEntryModePhoto4x3 ? AVCaptureSessionPresetPhoto : AVCaptureSessionPreset1280x720;
    
    AVCaptureSessionPreset present = _cameraMode == CameraEntryModeVideo ? videoPresent : imagePresent;
    if ([captureSession canSetSessionPreset:present]) {
        [captureSession setSessionPreset:present];
    }
    
    _inputCamera = [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeBuiltInWideAngleCamera, AVCaptureDeviceTypeBuiltInTelephotoCamera] mediaType:AVMediaTypeVideo position:_location].devices.firstObject;
    
    NSError* error;
    videoInput = [[AVCaptureDeviceInput alloc] initWithDevice:_inputCamera error:&error];
    if (error != nil) {
        _inputCamera = oldDevice;
        videoInput = oldDeviceInput;
    }
    
    if ([captureSession canAddInput:videoInput]) {
        [captureSession addInput:videoInput];
    }
    AVCaptureConnection* connection = [videoOutput connectionWithMediaType:AVMediaTypeVideo];
    if (_location == AVCaptureDevicePositionFront) {
        if ([connection isVideoMirroringSupported]) {
            [connection setVideoMirrored:_isFrontMirrord];
        }
    }else{
        if ([connection isVideoMirroringSupported]) {
            [connection setVideoMirrored:_isBackMirrord];
        }
    }
    
    [captureSession commitConfiguration];
    
}
- (void)configureDeviceFlash:(AVCaptureFlashMode) mode{
    if (_inputCamera.flashMode == mode) {
        return;
    }
    
    
    
    NSError* error;
    
    [_inputCamera lockForConfiguration:&error];
    if (error == nil){
        if ([_inputCamera isFlashModeSupported:mode]){
            [_inputCamera setFlashMode:mode];
        }
    }else{
        NSLog(@"%@",error);
    }
    [_inputCamera lockForConfiguration:&error];
    
    
}
- (void)configureDeviceTorch:(AVCaptureTorchMode) mode{
    
    
    
    if (_inputCamera.torchMode == mode) {
        return;
    }
    
    
    
    NSError* error;
    [_inputCamera lockForConfiguration:&error];
    if ([_inputCamera isTorchModeSupported:mode]){
        [_inputCamera setTorchMode:mode];
    }
    [_inputCamera lockForConfiguration:&error];
    
    
}
- (void) updateMirror
{
    AVCaptureConnection* connection = [videoOutput connectionWithMediaType:AVMediaTypeVideo];
    
    if (_location == AVCaptureDevicePositionBack) {
        if ([connection isVideoMirrored] == _isBackMirrord) {
            return;
        }
    }else if (_location == AVCaptureDevicePositionFront){
        
        
        if ([connection isVideoMirrored] == _isFrontMirrord) {
            return;
        }
        
    }
    [captureSession beginConfiguration];
    
    if (_location == AVCaptureDevicePositionFront) {
        if ([connection isVideoMirroringSupported]) {
            [connection setVideoMirrored:_isFrontMirrord];
        }
    }else{
        if ([connection isVideoMirroringSupported]) {
            [connection setVideoMirrored:_isBackMirrord];
        }
    }
    
    [captureSession commitConfiguration];
    
}


- (void) changeMode:(CameraEntryMode) mode
{
    _cameraMode = mode;
    [captureSession beginConfiguration];
    
    
    AVCaptureSessionPreset videoPresent = (_location == AVCaptureDevicePositionBack ? ([XLHelpClass isLowerThaniPhone6] ? AVCaptureSessionPreset1280x720 : AVCaptureSessionPreset1920x1080) : AVCaptureSessionPreset1280x720);
    AVCaptureSessionPreset imagePresent = mode == CameraEntryModePhoto4x3 ? AVCaptureSessionPresetPhoto : AVCaptureSessionPreset1280x720;
    
    AVCaptureSessionPreset present = _cameraMode == CameraEntryModeVideo ? videoPresent : imagePresent;
    if ([captureSession canSetSessionPreset:present]) {
        [captureSession setSessionPreset:present];
    }
    
    [captureSession commitConfiguration];
}

#pragma mark -- 拍照

#ifdef USE_STILLIMAGE

- (void) takePhotoWithCompletionHandle:(void (^)(CVPixelBufferRef,NSError*)) block
{
    
    
    
    dispatch_semaphore_wait(frameRenderingSemaphore, DISPATCH_TIME_FOREVER);
    
    
    
    [imageOutput captureStillImageAsynchronouslyFromConnection:[[imageOutput connections] objectAtIndex:0] completionHandler:^(CMSampleBufferRef  _Nullable imageDataSampleBuffer, NSError * _Nullable error) {
        
        if (error) {
            block(NULL,error);
        }else{
            CVPixelBufferRef buffer = CMSampleBufferGetImageBuffer(imageDataSampleBuffer);
            
            
            int width = (int)CVPixelBufferGetWidth(buffer);
            int height = (int)CVPixelBufferGetHeight(buffer);
            
            
            if (width <= 4096) {
                CFRetain(buffer);
                block(buffer,error);
                CFRelease(buffer);
            }else{
                if (width > 4096) {
                    width = 4096;
                    height = 4096 * height / width;
                }
                
                CVPixelBufferRef pixebuffer = NULL;
                
                GPUImageCreateResizedSampleBuffer(buffer, CGSizeMake(width, height), &pixebuffer);
                
                block(pixebuffer,error);
                
                CFRelease(pixebuffer);
            }
            
            
        }
        
        
        
        
        
        dispatch_semaphore_signal(self->frameRenderingSemaphore);
        
        
        
    }];
    
    
    
    
}
#else

- (AVCapturePhotoSettings*) createPhotoSettings
{
    AVCapturePhotoSettings* photoSettings = [AVCapturePhotoSettings photoSettingsWithFormat:[NSDictionary dictionaryWithObject:AVVideoCodecJPEG forKey:(id)AVVideoCodecKey]];
    return photoSettings;
}
- (void)takePhotoWithCompletionHandle:(void (^)(UIImage *))block
{
    completeImageHandle = block;
    AVCapturePhotoSettings* settings = [self createPhotoSettings];
    [imageOutput capturePhotoWithSettings:settings delegate:self];
}
- (void)captureOutput:(AVCapturePhotoOutput *)output didFinishProcessingPhotoSampleBuffer:(CMSampleBufferRef)photoSampleBuffer previewPhotoSampleBuffer:(CMSampleBufferRef)previewPhotoSampleBuffer resolvedSettings:(AVCaptureResolvedPhotoSettings *)resolvedSettings bracketSettings:(AVCaptureBracketedStillImageSettings *)bracketSettings error:(NSError *)error
{
    NSData* data = [AVCapturePhotoOutput JPEGPhotoDataRepresentationForJPEGSampleBuffer:photoSampleBuffer previewPhotoSampleBuffer:previewPhotoSampleBuffer];
    UIImage* image = [UIImage imageWithData:data];
    
    completeImageHandle(image);
}

#endif

#pragma mark -- 视频流 音频流

- (void)captureOutput:(AVCaptureOutput *)output didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer fromConnection:(AVCaptureConnection *)connection
{
    @autoreleasepool {
        if (!captureSession.isRunning) {
            [self changeMode:_cameraMode];
            return;
        }else if (output == audioOutput){
            
            if (self.delegate) {
                [self.delegate processAudioBuffer:sampleBuffer];
            }
            
            if (_audioInput) {
                [_audioInput captureOutput:output didOutputSampleBuffer:sampleBuffer fromConnection:connection];
            }
            
        }else{
            if(dispatch_semaphore_wait(frameRenderingSemaphore, DISPATCH_TIME_FOREVER)!=0){
                return;
            }
            CFRetain(sampleBuffer);
            
            runAsynchronouslyOnVideoProcessingQueue(^{
                if (self.delegate) {
                    [self.delegate processVideoBuffer:sampleBuffer];
                }
                
                if (_cameraInput) {
                    [_cameraInput captureOutput:output didOutputSampleBuffer:sampleBuffer fromConnection:connection];
                }
                
                
                
                CFRelease(sampleBuffer);
                
                if (self.logFPS) {
                    if ((CFAbsoluteTimeGetCurrent() - self.lastCheckTime) > 1.0)  {
                        self.lastCheckTime = CFAbsoluteTimeGetCurrent();
                        NSLog(@"FPS : %d",self.framesSinceLastCheck);
                        self.framesSinceLastCheck = 0;
                    }
                    self.framesSinceLastCheck += 1;
                }
                
                
                dispatch_semaphore_signal(frameRenderingSemaphore);
            });
        }
        
    }
}
@end

