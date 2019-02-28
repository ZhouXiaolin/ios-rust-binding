//
//  MovieWriter.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2019/1/22.
//  Copyright © 2019 Solaren. All rights reserved.
//

#import "MovieWriter.h"
#import <AVFoundation/AVFoundation.h>
#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "XHImageContext.h"

#define VIDEO_TIMESCALE 600

@interface MovieWriter()
{
    
    AVAssetWriter *assetWriter;
    AVAssetWriterInputPixelBufferAdaptor *assetWriterPixelBufferInput;
    AVAssetWriterInput* assetWriterVideoInput;
    AVAssetWriterInput* assetWriterAudioInput;
    NSString *pathToMovie;
    
    CMTime startTime, previousFrameTime, previousAudioTime;
    CMTime offsetTime;
    
    
    
    BOOL audioEncodingIsFinished, videoEncodingIsFinished;
    
    XHImageContext* _movieWriterContext;
    
    BOOL _encodingLiveVideo;
    BOOL alreadyFinishedRecording;
    
    
}
@end
@implementation MovieWriter


- (instancetype)initWithFrameSize:(CGSize) frameSize movieURL:(NSURL*)movieURL
{
    
    if (!(self=[super init])) {
        return nil;
    }
    
    startTime = kCMTimeInvalid;
    
    
    
    AVFileType fileType = AVFileTypeQuickTimeMovie;
    
    _movieWriterContext = [XHImageContext sharedImageProcessingContext];
    
    
    NSError *error = nil;
    assetWriter = [[AVAssetWriter alloc] initWithURL:movieURL fileType:fileType error:&error];
    
    if(error) {
        NSLog(@"error creating AssetWriter: %@",[error description]);
    }
    NSDictionary *videoSettings = [NSDictionary dictionaryWithObjectsAndKeys:
                                   AVVideoCodecH264, AVVideoCodecKey,
                                   [NSNumber numberWithInt:frameSize.width], AVVideoWidthKey,
                                   [NSNumber numberWithInt:frameSize.height], AVVideoHeightKey,
                                   nil];
    
    
    
    assetWriterVideoInput = [AVAssetWriterInput
                             assetWriterInputWithMediaType:AVMediaTypeVideo
                             outputSettings:videoSettings];
    
    NSMutableDictionary *attributes = [[NSMutableDictionary alloc] init];
    [attributes setObject:[NSNumber numberWithUnsignedInt:kCVPixelFormatType_32BGRA] forKey:(NSString*)kCVPixelBufferPixelFormatTypeKey];
    [attributes setObject:[NSNumber numberWithUnsignedInt:frameSize.width] forKey:(NSString*)kCVPixelBufferWidthKey];
    [attributes setObject:[NSNumber numberWithUnsignedInt:frameSize.height] forKey:(NSString*)kCVPixelBufferHeightKey];
    
    assetWriterPixelBufferInput = [AVAssetWriterInputPixelBufferAdaptor
                                   assetWriterInputPixelBufferAdaptorWithAssetWriterInput:assetWriterVideoInput
                                   sourcePixelBufferAttributes:attributes];
    
    [assetWriter addInput:assetWriterVideoInput];
    
    // fixes all errors
    assetWriterVideoInput.expectsMediaDataInRealTime = YES;
    
    
    return self;
}

- (void) start {
    startTime = kCMTimeInvalid;
    runSynchronouslyOnContextQueue(_movieWriterContext, ^{
        [assetWriter startWriting];
    });
    _isRecording = YES;
}
- (void) stop {
    [self stop:^{
        
    }];
}
- (void) stop:(void (^)(void))handler
{
    runSynchronouslyOnContextQueue(_movieWriterContext, ^{
        _isRecording = FALSE;
        _isReady = FALSE;
        _frameCount = 0;
        
        if (assetWriter.status == AVAssetWriterStatusCompleted || assetWriter.status == AVAssetWriterStatusCancelled || assetWriter.status == AVAssetWriterStatusUnknown) {
                    if (handler) {
                        runAsynchronouslyOnContextQueue(_movieWriterContext, handler);
                    }
            return;
        }
        
        if (assetWriter.status == AVAssetWriterStatusWriting && !videoEncodingIsFinished) {
            videoEncodingIsFinished = TRUE;
            [assetWriterVideoInput markAsFinished];
        }
        
        if (assetWriter.status == AVAssetWriterStatusWriting && !audioEncodingIsFinished) {
            audioEncodingIsFinished = TRUE;
            [assetWriterAudioInput markAsFinished];
        }
        
        if (handler) {
            [assetWriter finishWritingWithCompletionHandler:handler];
        }else{
            [assetWriter finishWriting];
        }
    });
}


- (void) readAndPutWithWidth:(int)width height:(int)height frameTime:(CMTime)frameTime;
{
    if (!_isRecording) {
        return;
    }
    
    int _width = width;
    int _height = height;
    
    if (CMTIME_IS_INVALID(startTime)) {
        runSynchronouslyOnContextQueue(_movieWriterContext, ^{
            if (assetWriter.status != AVAssetWriterStatusWriting) {
                [assetWriter startWriting];
            }
            [assetWriter startSessionAtSourceTime:frameTime];
            
            startTime = frameTime;
        });
        
    }
    
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
    
    
    runAsynchronouslyOnContextQueue(_movieWriterContext, ^{
        
        
        if ([[assetWriterPixelBufferInput assetWriterInput] isReadyForMoreMediaData]) {
            //
            BOOL result = [assetWriterPixelBufferInput appendPixelBuffer:pxbuffer withPresentationTime:frameTime];
            
            if (result == NO) //failes on 3GS, but works on iphone 4
            {
                NSLog(@"failed to append buffer");
                NSLog(@"The error is %@", [assetWriter error]);
            }
        }
        
        
        CVPixelBufferUnlockBaseAddress(pxbuffer, 0);
        CVPixelBufferRelease(pxbuffer);
    });
    
    
}

@end

