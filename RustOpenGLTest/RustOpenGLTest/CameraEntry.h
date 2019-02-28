//
//  CameraEntryObjC.h
//  XCamera
//
//  Created by 周晓林 on 2018/7/30.
//  Copyright © 2018年 xhey. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#define USE_STILLIMAGE // 这个宏需要和other swift flags中的同名符号配合使用

typedef NS_ENUM(NSInteger, CameraEntryMode){
    CameraEntryModeVideo = 0,
    CameraEntryModePhoto4x3 = 1,
    CameraEntryModePhoto16x9 = 2
};


@protocol CameraEntryDelegate <NSObject>

- (void) processAudioBuffer:(CMSampleBufferRef) audioBuffer;
- (void) processVideoBuffer:(CMSampleBufferRef) videoBuffer;

@end

@interface CameraEntry : NSObject
@property (nonatomic, assign) CameraEntryMode cameraMode;
@property (nonatomic, assign) AVCaptureDevicePosition location;
@property (nonatomic, assign) BOOL isBackMirrord;
@property (nonatomic, assign) BOOL isFrontMirrord;
@property (nonatomic, assign) BOOL logFPS;
@property (nonatomic, strong) AVCaptureDevice* inputCamera;
@property (nonatomic, assign) OSType pixelFormatType;
@property (nonatomic, assign) id<CameraEntryDelegate> delegate;

- (instancetype)initWithSessionPreset:(AVCaptureSessionPreset) sessionPreset
                             location:(AVCaptureDevicePosition) location
                      cameraEntryMode:(CameraEntryMode)cameraEntryMode
                         captureAsYUV:(BOOL)captureAsYUV;
- (void) addAudioInputsAndOutputsWithDelegate:(id<AVCaptureAudioDataOutputSampleBufferDelegate>) delegate;
- (void) addAudioInputsAndOutputs;
- (void) removeAudioInputsAndOutputs;
- (void) setVideoOutputDelegate:(id<AVCaptureVideoDataOutputSampleBufferDelegate>) delegate;
- (void) startCapture;
- (void) stopCapture;
#ifdef USE_STILLIMAGE
- (void) takePhotoWithCompletionHandle:(void (^)(CVPixelBufferRef,NSError*)) block;
#else
- (void) takePhotoWithCompletionHandle:(void (^)(UIImage*)) block;
#endif
- (void) changeMode:(CameraEntryMode) mode;
- (void) setStabilitizationMode:(AVCaptureVideoStabilizationMode) mode;
- (void) focusAtPoint:(CGPoint) point isSmooth:(BOOL) isSmooth focusMode:(AVCaptureFocusMode) mode;
- (void) exposureAtPoint:(CGPoint) point exposureMode:(AVCaptureExposureMode) mode;
- (void) setExposure:(float) bias;
- (void) zoomBegin:(CGFloat) scale;
- (void) zoomEnd:(CGFloat) scale;
- (void) zoomWithScale:(CGFloat) scale;
- (void)configureDeviceFlash:(AVCaptureFlashMode) mode;
- (void)configureDeviceTorch:(AVCaptureTorchMode) mode;
- (BOOL)isRunning;

@end
