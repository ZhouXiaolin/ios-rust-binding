//
//  XHImageContext.h
//  XCamera
//
//  Created by 周晓林 on 2019/2/15.
//  Copyright © 2019 xhey. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
#import <OpenGLES/EAGL.h>

@class XHImageContext;

void runSynchronouslyOnVideoProcessingQueue(void (^block)(void));
void runAsynchronouslyOnVideoProcessingQueue(void (^block)(void));
void runSynchronouslyOnContextQueue(XHImageContext *context, void (^block)(void));
void runAsynchronouslyOnContextQueue(XHImageContext *context, void (^block)(void));




NS_ASSUME_NONNULL_BEGIN

@interface XHImageContext : NSObject
@property(readonly, retain, nonatomic) EAGLContext *context;
@property(readonly, nonatomic) dispatch_queue_t contextQueue;
@property(readonly) CVOpenGLESTextureCacheRef coreVideoTextureCache;

+ (XHImageContext *)sharedImageProcessingContext;
+ (void)useImageProcessingContext;
+ (dispatch_queue_t)sharedContextQueue;
+ (void *)contextKey;

@end

NS_ASSUME_NONNULL_END

