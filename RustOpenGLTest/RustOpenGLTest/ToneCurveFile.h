//
//  GPUImageACVFile.h
//  GXTest
//
//  Created by 周晓林 on 2018/8/23.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>
@interface ToneCurveFile : NSObject{
    short version;
    short totalCurves;
    
    NSArray *rgbCompositeCurvePoints;
    NSArray *redCurvePoints;
    NSArray *greenCurvePoints;
    NSArray *blueCurvePoints;
}

@property(strong,nonatomic) NSArray *rgbCompositeCurvePoints;
@property(strong,nonatomic) NSArray *redCurvePoints;
@property(strong,nonatomic) NSArray *greenCurvePoints;
@property(strong,nonatomic) NSArray *blueCurvePoints;

- (id) initWithToneCurveFileData:(NSData*)data;


unsigned short int16WithBytes(Byte* bytes);
@end
