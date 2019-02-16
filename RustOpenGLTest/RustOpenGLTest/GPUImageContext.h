//
//  GPUImageContext.h
//  RustOpenGLTest
//
//  Created by 周晓林 on 2019/2/15.
//  Copyright © 2019 Solaren. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <OpenGLES/EAGL.h>

NS_ASSUME_NONNULL_BEGIN

@interface GPUImageContext : NSObject
@property(readonly, retain, nonatomic) EAGLContext *context;
+ (GPUImageContext *)sharedImageProcessingContext;
+ (void)useImageProcessingContext;
@end

NS_ASSUME_NONNULL_END
