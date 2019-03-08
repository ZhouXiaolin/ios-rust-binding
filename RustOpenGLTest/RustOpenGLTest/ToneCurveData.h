//
//  GPUImageACVTexture.h
//  GXTest
//
//  Created by 周晓林 on 2018/8/28.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import <Foundation/Foundation.h>
@interface ToneCurveData : NSObject
@property (readonly) int width;
@property (readonly) int height;
@property (readonly) uint8_t *textureData;
- (id)initWithACVData:(NSData *)data;
- (id)initWithName:(NSString*)curveFilename;
- (id)initWithACVURL:(NSURL*)curveFileURL;
@end
