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
    
    _frameCount = 0;
    startTime = kCMTimeInvalid;
    videoEncodingIsFinished = NO;
    audioEncodingIsFinished = NO;
    
    previousFrameTime = kCMTimeNegativeInfinity;
    previousAudioTime = kCMTimeNegativeInfinity;
    _movieWriterContext = [XHImageContext sharedImageProcessingContext];
    _encodingLiveVideo = YES;
    
    AVFileType fileType = AVFileTypeQuickTimeMovie;
    
    
    
    NSError *error = nil;
    assetWriter = [[AVAssetWriter alloc] initWithURL:movieURL fileType:fileType error:&error];
    
    if(error != nil) {
        NSLog(@"error creating AssetWriter: %@",[error description]);
        if (_errorBlock) {
            _errorBlock(error);
        }
    }
    assetWriter.movieFragmentInterval = CMTimeMakeWithSeconds(1.0, VIDEO_TIMESCALE);
    assetWriter.movieTimeScale = VIDEO_TIMESCALE;
    
    
    NSDictionary *videoSettings = [NSDictionary dictionaryWithObjectsAndKeys:
                                   AVVideoCodecH264, AVVideoCodecKey,
                                   [NSNumber numberWithInt:frameSize.width], AVVideoWidthKey,
                                   [NSNumber numberWithInt:frameSize.height], AVVideoHeightKey,
                                   nil];
    
    
    
    assetWriterVideoInput = [AVAssetWriterInput
                             assetWriterInputWithMediaType:AVMediaTypeVideo
                             outputSettings:videoSettings];
    // fixes all errors
    assetWriterVideoInput.expectsMediaDataInRealTime = _encodingLiveVideo;
    assetWriterVideoInput.mediaTimeScale = VIDEO_TIMESCALE;
    
    NSMutableDictionary *attributes = [[NSMutableDictionary alloc] init];
    [attributes setObject:[NSNumber numberWithUnsignedInt:kCVPixelFormatType_32BGRA] forKey:(NSString*)kCVPixelBufferPixelFormatTypeKey];
    [attributes setObject:[NSNumber numberWithUnsignedInt:frameSize.width] forKey:(NSString*)kCVPixelBufferWidthKey];
    [attributes setObject:[NSNumber numberWithUnsignedInt:frameSize.height] forKey:(NSString*)kCVPixelBufferHeightKey];
    
    assetWriterPixelBufferInput = [AVAssetWriterInputPixelBufferAdaptor
                                   assetWriterInputPixelBufferAdaptorWithAssetWriterInput:assetWriterVideoInput
                                   sourcePixelBufferAttributes:attributes];
    
    [assetWriter addInput:assetWriterVideoInput];
    
    
    
    
    return self;
}


- (void) setMovieWriterTransform:(CGAffineTransform) transform
{
    assetWriterVideoInput.transform = transform;
    
}

- (void) start {
    
    
    _frameCount = 0;
    startTime = kCMTimeInvalid;
    runSynchronouslyOnContextQueue(_movieWriterContext, ^{
        [assetWriter startWriting];
    });
    _isRecording = YES;
}

- (void) stop{
    [self stop:^{
        
    }];
}

- (void) processAudioBuffer:(CMSampleBufferRef)sampleBuffer
{
    
#if 1
    if (!_isRecording) {
        return;
    }
    
    // 先录制视频帧
    if (CMTIME_IS_INVALID(startTime)) {
        return;
    }
    
    CFRetain(sampleBuffer);
    CMTime currentSampleTime = CMSampleBufferGetOutputPresentationTimeStamp(sampleBuffer);
    
    if (CMTIME_IS_INVALID(startTime)) {
        
        runSynchronouslyOnContextQueue(_movieWriterContext, ^{
            if (assetWriter.status != AVAssetWriterStatusWriting) {
                [assetWriter startWriting];
            }
            [assetWriter startSessionAtSourceTime:currentSampleTime];
            
            startTime = currentSampleTime;
        });
    }
    
    
    if (!assetWriterAudioInput.readyForMoreMediaData && _encodingLiveVideo) {
        CFRelease(sampleBuffer);
        return;
    }
    
    void(^write)() = ^(){
        while (!assetWriterAudioInput.readyForMoreMediaData && !_encodingLiveVideo && !audioEncodingIsFinished) {
            NSDate *maxDate = [NSDate dateWithTimeIntervalSinceNow:0.5];
            [[NSRunLoop currentRunLoop] runUntilDate:maxDate];
        }
        
        if (!assetWriterAudioInput.readyForMoreMediaData) {
            NSLog(@"2: Had to drop an audio frame %@", CFBridgingRelease(CMTimeCopyDescription(kCFAllocatorDefault, currentSampleTime)));
            
        }else if (assetWriter.status == AVAssetWriterStatusWriting){
            if (![assetWriterAudioInput appendSampleBuffer:sampleBuffer]) {
                NSLog(@"Problem appending audio buffer at time: %@", CFBridgingRelease(CMTimeCopyDescription(kCFAllocatorDefault, currentSampleTime)));
                
            }
        }else{
            
        }
        
        CFRelease(sampleBuffer);
        
    };
    
    if (_encodingLiveVideo) {
        runAsynchronouslyOnContextQueue(_movieWriterContext, write);
    }else{
        write();
    }
#endif
}
- (void)dealloc {
    NSLog(@"->->->->->->->->->->->->->->->->->->->->->->->->->->->->->->");
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

- (void) activateAudioTrack
{
    NSDictionary* audioOutputSettings = [NSDictionary dictionaryWithObjectsAndKeys:
                                         [NSNumber numberWithInt:kAudioFormatMPEG4AAC_HE],AVFormatIDKey,
                                         [NSNumber numberWithInt:2], AVNumberOfChannelsKey,
                                         [NSNumber numberWithFloat:44100], AVSampleRateKey,
                                         [NSNumber numberWithInt:64000], AVEncoderBitRateKey,
                                         nil];
    assetWriterAudioInput = [[AVAssetWriterInput alloc] initWithMediaType:AVMediaTypeAudio outputSettings:audioOutputSettings];
    [assetWriter addInput:assetWriterAudioInput];
    [assetWriterAudioInput setExpectsMediaDataInRealTime:_encodingLiveVideo];
}

- (void) cancelRecording {
    if (assetWriter.status == AVAssetWriterStatusCompleted) {
        return;
    }
    
    _isRecording = NO;
    runSynchronouslyOnContextQueue(_movieWriterContext, ^{
        alreadyFinishedRecording = YES;
        if (assetWriter.status == AVAssetWriterStatusWriting && !videoEncodingIsFinished) {
            videoEncodingIsFinished = YES;
            [assetWriterVideoInput markAsFinished];
        }
        
        if (assetWriter.status == AVAssetWriterStatusWriting && !audioEncodingIsFinished) {
            audioEncodingIsFinished = YES;
            [assetWriterAudioInput markAsFinished];
        }
        [assetWriter cancelWriting];
    });
}


- (void) readAndPutWithWidth:(int)width height:(int)height frameTime:(CMTime)frameTime
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
    
    CMTime currentTime = CMTimeSubtract(frameTime, startTime);
    CGFloat cur_t = CMTimeGetSeconds(currentTime);
    
    _frameCount++;
    
    
    if (self.progressBlock) {
        self.progressBlock(cur_t);
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

