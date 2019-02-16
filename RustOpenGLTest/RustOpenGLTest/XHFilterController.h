//
//  FilterController.h
//  RustOpenGLTest
//
//  Created by 周晓林 on 2019/1/22.
//  Copyright © 2019 Solaren. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import <UIKit/UIKit.h>
NS_ASSUME_NONNULL_BEGIN

typedef NS_ENUM(NSInteger, XHFilterControllerMode){
    XHFilterControllerModePhotoFront   = 0,
    XHFilterControllerModePhotoBack    = 1,
    XHFilterControllerModeVideoFront   = 2,
    XHFilterControllerModeVideoBack    = 3,
    XHFilterControllerModePhotoProcess = 4,
    XHFilterControllerModeVideoProcess = 5
};

@interface WaterViewInfo : NSObject
@property (nonatomic, assign) CGFloat x;
@property (nonatomic, assign) CGFloat y;
@property (nonatomic, assign) CGFloat w;
@property (nonatomic, assign) CGFloat h;
@end


@class CameraEntry;
@class OpenGLView;
@class MovieWriter;

@interface XHFilterController : NSObject<AVCaptureVideoDataOutputSampleBufferDelegate>

- (instancetype)initWithInput:(CameraEntry*) cameraEntry
                   renderView:(OpenGLView*)glView
                       writer:(MovieWriter*)movieWriter;


- (instancetype)initWithPicture:(UIImage*) image
                     renderView:(OpenGLView*)glView;

- (void) renderPictureWithLut:(NSString*)lut;


- (void) changeFilter:(XHFilterControllerMode) mode;
- (void) changeLookup:(NSString*) path;
- (void) switchCamera;
- (void) startCapture;

- (void) stopCapture;

- (void) startRecordWithWaterInfo:(WaterViewInfo*)waterInfo destinationURL:(NSURL*)url;

- (void) stopRecordWithCompletion:(void (^)(NSError*))handler;

- (void) capturePhotoWithWater: (WaterViewInfo*)waterInfo
            previewImgCallBack: (void (^)(UIImage* img, NSError* error))previewImgCallBack
           originalImgCallBack: (void (^)(UIImage* img, NSError* error))originalImgCallBack
          processedImgCallBack: (void (^)(UIImage* img, NSError* error))processedImgCallBack;


@end

NS_ASSUME_NONNULL_END
