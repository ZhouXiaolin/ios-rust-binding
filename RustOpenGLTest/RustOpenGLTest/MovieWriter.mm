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
@interface MovieWriter()
{
    BOOL isRecording;
    
    AVAssetWriter *videoWriter;
    AVAssetWriterInputPixelBufferAdaptor *adaptor;
    AVAssetWriterInput* writerInput;
    NSString *pathToMovie;
    
    CMTime startTime;
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
    
    
    
    NSError *error = nil;
    videoWriter = [[AVAssetWriter alloc] initWithURL:movieURL fileType:fileType error:&error];
    
    if(error) {
        NSLog(@"error creating AssetWriter: %@",[error description]);
    }
    NSDictionary *videoSettings = [NSDictionary dictionaryWithObjectsAndKeys:
                                   AVVideoCodecH264, AVVideoCodecKey,
                                   [NSNumber numberWithInt:frameSize.width], AVVideoWidthKey,
                                   [NSNumber numberWithInt:frameSize.height], AVVideoHeightKey,
                                   nil];
    
    
    
    writerInput = [AVAssetWriterInput
                   assetWriterInputWithMediaType:AVMediaTypeVideo
                   outputSettings:videoSettings];
    
    NSMutableDictionary *attributes = [[NSMutableDictionary alloc] init];
    [attributes setObject:[NSNumber numberWithUnsignedInt:kCVPixelFormatType_32BGRA] forKey:(NSString*)kCVPixelBufferPixelFormatTypeKey];
    [attributes setObject:[NSNumber numberWithUnsignedInt:frameSize.width] forKey:(NSString*)kCVPixelBufferWidthKey];
    [attributes setObject:[NSNumber numberWithUnsignedInt:frameSize.height] forKey:(NSString*)kCVPixelBufferHeightKey];
    
    adaptor = [AVAssetWriterInputPixelBufferAdaptor
               assetWriterInputPixelBufferAdaptorWithAssetWriterInput:writerInput
               sourcePixelBufferAttributes:attributes];
    
    [videoWriter addInput:writerInput];
    
    // fixes all errors
    writerInput.expectsMediaDataInRealTime = YES;
    
    
    return self;
}

- (void) start {
    isRecording = YES;
    
    BOOL start = [videoWriter startWriting];
    [videoWriter startSessionAtSourceTime:kCMTimeZero];
}

- (void) stop{
    isRecording = FALSE;
    
    [writerInput markAsFinished];
    [videoWriter finishWritingWithCompletionHandler:^{

    }];
}

- (void) readAndPut:(int)height width:(int)width frameTime:(CMTime)frameTime
{
    if (isRecording == YES) {
        
        
        int _width = height;
        int _height = width;
        
        if (CMTIME_IS_INVALID(startTime)) {
            startTime = frameTime;
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
        
        
        if ([[adaptor assetWriterInput] isReadyForMoreMediaData]) {
            BOOL result = [adaptor appendPixelBuffer:pxbuffer withPresentationTime:CMTimeSubtract(frameTime, startTime)];
            
            if (result == NO) //failes on 3GS, but works on iphone 4
            {
                NSLog(@"failed to append buffer");
                NSLog(@"The error is %@", [videoWriter error]);
            }
        }
        
        
        CVPixelBufferUnlockBaseAddress(pxbuffer, 0);
        CVPixelBufferRelease(pxbuffer);
        
        
    }
}

@end
