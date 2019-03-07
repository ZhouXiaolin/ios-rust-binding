//
//  GPUImageACVTexture.h
//  GXTest
//
//  Created by 周晓林 on 2018/8/28.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
@interface ToneCurveData : NSObject
@property (readonly) int width;
@property (readonly) int height;
@property (readonly) GLubyte *textureData;
- (id)initWithACVData:(NSData *)data;
- (id)initWithName:(NSString*)curveFilename;
- (id)initWithACVURL:(NSURL*)curveFileURL;
@end
