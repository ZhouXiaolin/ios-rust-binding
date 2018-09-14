//
//  CameraEntryObjC.m
//  XCamera
//
//  Created by 周晓林 on 2018/7/30.
//  Copyright © 2018年 xhey. All rights reserved.
//

#import "CameraEntry.h"
#import "XLHelpClass.h"


@interface CameraEntry () <AVCapturePhotoCaptureDelegate,AVCaptureVideoDataOutputSampleBufferDelegate>
{
    
    
    float preScale;
    AVCaptureSession *captureSession;
    AVCaptureDeviceInput *videoInput;
    AVCaptureVideoDataOutput *videoOutput;
    dispatch_queue_t cameraQueue;
    
    AVCaptureDevice *microphone;
    
#ifdef USE_STILLIMAGE
    AVCaptureStillImageOutput* imageOutput;
#else
    
#endif
    
    AVCaptureDeviceInput* audioInput;
    AVCaptureAudioDataOutput* audioOutput;
    
    dispatch_semaphore_t frameRenderingSemaphore;
    BOOL isFullYUVRange;
    BOOL isPhoto;
    
    id<AVCaptureVideoDataOutputSampleBufferDelegate> _cameraInput;

}
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
    
    [_inputCamera setSubjectAreaChangeMonitoringEnabled:TRUE]; // 开启范围监测
    
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
    
    if ([_inputCamera isVideoHDREnabled]) {
        [_inputCamera setVideoHDREnabled:YES];
    }
    
    if ([_inputCamera isLowLightBoostSupported]) { // 弱光下自动提升亮度
        [_inputCamera setAutomaticallyEnablesLowLightBoostWhenAvailable:TRUE];
    }
    
    
    [_inputCamera unlockForConfiguration];
}
- (instancetype)initWithSessionPreset:(AVCaptureSessionPreset) sessionPreset
                             location:(AVCaptureDevicePosition) location
                         captureAsYUV:(BOOL)captureAsYUV
{
    if (!(self = [super init])) {
        return nil;
    }
    preScale = 0.0;
    _cameraMode = CameraEntryModeVideo;
    
    cameraQueue = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH,0);
    
    
    self.location = location;
    
    frameRenderingSemaphore = dispatch_semaphore_create(1);
    
    _inputCamera = [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeBuiltInWideAngleCamera, AVCaptureDeviceTypeBuiltInTelephotoCamera] mediaType:AVMediaTypeVideo position:location].devices.firstObject;
    
    if (!_inputCamera) {
        return nil;
    }
    
    [self setCameraAdjustParameter];

    
    captureSession = [[AVCaptureSession alloc] init];
    [captureSession beginConfiguration];
    
    NSError* error = nil;
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
    
    
    imageOutput = [[AVCaptureStillImageOutput alloc] init];
    [imageOutput setOutputSettings:[NSDictionary dictionaryWithObject:[NSNumber numberWithInt:kCVPixelFormatType_420YpCbCr8BiPlanarFullRange] forKey:(id)kCVPixelBufferPixelFormatTypeKey]];
    if ([captureSession canAddOutput:imageOutput]) {
        [captureSession addOutput:imageOutput];
    }
    
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
    if (![captureSession isRunning]) {
        [captureSession startRunning];
    }
    
}

- (void) stopCapture {
    if ([captureSession isRunning]) {
        [captureSession stopRunning];
    }
}

- (BOOL) canSwitchCameras
{
    return [AVCaptureDeviceDiscoverySession discoverySessionWithDeviceTypes:@[AVCaptureDeviceTypeBuiltInWideAngleCamera,AVCaptureDeviceTypeBuiltInTelephotoCamera] mediaType:AVMediaTypeVideo position:AVCaptureDevicePositionUnspecified].devices.count > 1;
}

- (void) addAudioInputsAndOutputsWithDelegate:(id<AVCaptureAudioDataOutputSampleBufferDelegate>) delegate callbackQueue:(dispatch_queue_t) callbackQueue
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
    [audioOutput setSampleBufferDelegate:delegate queue:callbackQueue];
    
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
        [_inputCamera setFocusMode:AVCaptureFocusModeAutoFocus];
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
    AVCaptureSessionPreset imagePresent = AVCaptureSessionPresetPhoto;
    
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
    
    if ([_inputCamera isVideoHDREnabled]) {
        [_inputCamera setVideoHDREnabled:YES];
    }
    
    [captureSession commitConfiguration];
    
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
    AVCaptureSessionPreset imagePresent = AVCaptureSessionPresetPhoto;
    
    AVCaptureSessionPreset present = _cameraMode == CameraEntryModeVideo ? videoPresent : imagePresent;
    if ([captureSession canSetSessionPreset:present]) {
        [captureSession setSessionPreset:present];
    }
    [captureSession commitConfiguration];
}

#pragma mark -- 拍照

- (void)takePhotoProcessedUpToFilter:(void *)finalFilterInChain WithCompletionHandler:(void (^)(CVPixelBufferRef))block{
   
}

#if 0
- (void) takePhotoWithCompletionHandler:(void (^)(CVPixelBufferRef imagePixelBuffer)) block
{
    
    if (isPhoto) {
        isPhoto = false;
    }else{
        isPhoto = true;
    }
    
    AVCaptureConnection* connection = [[imageOutput connections] objectAtIndex:0];
    dispatch_semaphore_wait(frameRenderingSemaphore, DISPATCH_TIME_FOREVER);
    [imageOutput captureStillImageAsynchronouslyFromConnection:[[imageOutput connections] objectAtIndex:0] completionHandler:^(CMSampleBufferRef  _Nullable imageDataSampleBuffer, NSError * _Nullable error) {
        
        CVPixelBufferRef cameraFrame = CMSampleBufferGetImageBuffer(imageDataSampleBuffer);
        
        NSLog(@"solaren %zu %zu",CVPixelBufferGetWidth(cameraFrame), CVPixelBufferGetHeight(cameraFrame));
        
        CGSize finalSize = CGSizeMake(1000, 750);

        CIImage* image = [[CIImage alloc] initWithCVPixelBuffer:cameraFrame];
        image = [image imageByApplyingTransform:CGAffineTransformMakeScale(finalSize.width/CVPixelBufferGetWidth(cameraFrame), finalSize.height/CVPixelBufferGetHeight(cameraFrame))];
        image = [image imageByApplyingCGOrientation:(kCGImagePropertyOrientationUpMirrored)];
        
        CIContext* context = [CIContext contextWithOptions:nil];
        
        CVPixelBufferRef pixel_buffer = NULL;
        CVPixelBufferCreate(kCFAllocatorDefault, finalSize.width, finalSize.height, self.pixelFormatType, NULL, &pixel_buffer);
        [context render:image toCVPixelBuffer:pixel_buffer];
        
        CMVideoFormatDescriptionRef videoInfo = NULL;
        CMVideoFormatDescriptionCreateForImageBuffer(NULL, pixel_buffer, &videoInfo);
        
        CMTime frameTime = CMTimeMake(1, 30);
        CMSampleTimingInfo timing = {frameTime, frameTime, kCMTimeInvalid};
        
        
        CMSampleBufferRef sampleBuffer = NULL;
        CMSampleBufferCreateForImageBuffer(kCFAllocatorDefault, pixel_buffer, YES, NULL, NULL, videoInfo, &timing, &sampleBuffer);
        
        
        dispatch_semaphore_signal(self->frameRenderingSemaphore);
//        CFRetain(sampleBuffer);
        [self->_cameraInput captureOutput:self->imageOutput didOutputSampleBuffer:sampleBuffer fromConnection:connection];
//        CFRelease(sampleBuffer);
        dispatch_semaphore_wait(self->frameRenderingSemaphore, DISPATCH_TIME_FOREVER);

        CFRelease(sampleBuffer);
        CVPixelBufferRelease(pixel_buffer);
        
        
        dispatch_semaphore_signal(self->frameRenderingSemaphore);
        
        block(nil);

    }];


}
#endif

- (void) takePhotoWithCompletionHandle:(void (^)(CVPixelBufferRef)) block
{
    if(dispatch_semaphore_wait(frameRenderingSemaphore, DISPATCH_TIME_FOREVER)!=0){
        return;
    }

    [imageOutput captureStillImageAsynchronouslyFromConnection:[[imageOutput connections] objectAtIndex:0] completionHandler:^(CMSampleBufferRef  _Nullable imageDataSampleBuffer, NSError * _Nullable error) {
        CVPixelBufferRef buffer = CMSampleBufferGetImageBuffer(imageDataSampleBuffer);
        CFRetain(buffer);
        dispatch_async(self->cameraQueue, ^{
            block(buffer);
            CFRelease(buffer);
            dispatch_semaphore_signal(self->frameRenderingSemaphore);
        });
    }];
    
    
}


#pragma mark -- 视频流

- (void)captureOutput:(AVCaptureOutput *)output didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer fromConnection:(AVCaptureConnection *)connection
{
    if (isPhoto) {
        return;
    }
    
    if(dispatch_semaphore_wait(frameRenderingSemaphore, DISPATCH_TIME_FOREVER)!=0){
        return;
    }
    CFRetain(sampleBuffer);
    [_cameraInput captureOutput:output didOutputSampleBuffer:sampleBuffer fromConnection:connection];
    CFRelease(sampleBuffer);
    dispatch_semaphore_signal(frameRenderingSemaphore);
    
    
}
@end
