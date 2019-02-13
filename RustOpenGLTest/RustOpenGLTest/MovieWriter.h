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
- (instancetype)initWithFrameSize:(CGSize) frameSize movieURL:(NSURL*)movieURL;
- (void) start;
- (void) stop;
- (void) readAndPut:(int)height width:(int)width frameTime:(CMTime)frameTime;
@end

NS_ASSUME_NONNULL_END
