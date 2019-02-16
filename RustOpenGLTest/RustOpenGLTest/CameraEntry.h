//
//  CameraEntryObjC.h
//  XCamera
//
//  Created by 周晓林 on 2018/7/30.
//  Copyright © 2018年 xhey. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

typedef NS_ENUM(NSInteger, CameraEntryMode){
    CameraEntryModeVideo = 0,
    CameraEntryModePhoto = 1,
};

#define USE_STILLIMAGE
@interface CameraEntry : NSObject
@property (nonatomic, assign) CameraEntryMode cameraMode;
@property (nonatomic, assign) AVCaptureDevicePosition location;
@property (nonatomic, assign) BOOL isBackMirrord;
@property (nonatomic, assign) BOOL isFrontMirrord;
@property (nonatomic, strong) AVCaptureDevice* inputCamera;
@property (nonatomic, assign) OSType pixelFormatType;
- (instancetype)initWithSessionPreset:(AVCaptureSessionPreset) sessionPreset
                             location:(AVCaptureDevicePosition) location
                         captureAsYUV:(BOOL)captureAsYUV;
- (void) addAudioInputsAndOutputsWithDelegate:(id<AVCaptureAudioDataOutputSampleBufferDelegate>) delegate callbackQueue:(dispatch_queue_t) callbackQueue;
- (void) removeAudioInputsAndOutputs;
- (void) setVideoOutputDelegate:(id<AVCaptureVideoDataOutputSampleBufferDelegate>) delegate;
- (void) startCapture;
- (void) stopCapture;
#ifdef USE_STILLIMAGE
- (void) takePhotoWithCompletionHandle:(void (^)(CVPixelBufferRef imagePixelBuffer)) block;
- (void) takePhotoProcessedUpToFilter:(void *) finalFilterInChain WithCompletionHandler:(void (^)(CVPixelBufferRef))block;

#else
#endif
- (void) changeMode:(CameraEntryMode) mode;
- (void) setStabilitizationMode:(AVCaptureVideoStabilizationMode) mode;
- (void) focusAtPoint:(CGPoint) point isSmooth:(BOOL) isSmooth focusMode:(AVCaptureFocusMode) mode;
- (void) exposureAtPoint:(CGPoint) point exposureMode:(AVCaptureExposureMode) mode;

- (void) setExposure:(float) bias;
- (void) zoomBegin:(CGFloat) scale;
- (void) zoomEnd:(CGFloat) scale;
- (void) zoomWithScale:(CGFloat) scale;

@end
