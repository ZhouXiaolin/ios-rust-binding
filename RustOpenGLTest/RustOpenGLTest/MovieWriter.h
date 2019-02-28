//
//  MovieWriter.h
//  RustOpenGLTest
//
//  Created by 周晓林 on 2019/1/22.
//  Copyright © 2019 Solaren. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>
NS_ASSUME_NONNULL_BEGIN

@interface MovieWriter : NSObject
@property (nonatomic, assign) BOOL isReady;
@property (nonatomic, assign) BOOL isRecording;
@property (nonatomic, copy) void(^errorBlock)(NSError*);
@property (nonatomic, copy) void(^progressBlock)(CGFloat);
@property (nonatomic, assign) int frameCount;

- (instancetype)initWithFrameSize:(CGSize) frameSize movieURL:(NSURL*)movieURL;
- (void) start;
- (void) stop;
- (void) stop:(void (^)(void))handler;
- (void) readAndPutWithWidth:(int)width height:(int)height frameTime:(CMTime)frameTime;
- (void) processAudioBuffer:(CMSampleBufferRef)sampleBuffer;
- (void) activateAudioTrack;
- (void) setMovieWriterTransform:(CGAffineTransform) transform;

@end

NS_ASSUME_NONNULL_END
