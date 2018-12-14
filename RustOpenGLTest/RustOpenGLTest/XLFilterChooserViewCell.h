//
//  RDFilterChooserViewCell.h
//  RDVEUISDK
//
//  Created by 周晓林 on 16/4/8.
//
//

#import <UIKit/UIKit.h>
@interface XLFilter : NSObject

typedef enum {
    kRDFilterType_YuanShi = 0,        // 原始
    kRDFilterType_HeiBai,             // 黑白
    kRDFilterType_SuiYue,             // 岁月
    kRDFilterType_FanXiang,           // 反向
    kRDFilterType_BianYuan,           // 边缘
    kRDFilterType_NiuQu,              // 扭曲
    kRDFilterType_Turn,               // 反转
    kRDFilterType_SLBP,               // SLBP
    kRDFilterType_Sketch,             // 素描
    kRDFilterType_DistortingMirror,   // 哈哈镜
    kRDFilterType_ACV,                // acv滤镜
    kRDFilterType_LookUp              // lookup滤镜
} RDFilterType;

/**滤镜类型
 */
@property (nonatomic,assign)NSInteger type;

/**滤镜名称
 */
@property (nonatomic,copy  )NSString *name;

/**滤镜资源地址
 */
@property (nonatomic,copy  )NSString *filterPath;

@end


@class RDRecordGPUImageFilter;
@interface XLFilterChooserViewCell : UIView

@property (nonatomic , strong) UILabel *titleLabel;
@property (nonatomic , strong) UIImageView* backgroudView;
@property (nonatomic , strong) UIColor *circleColor;

- (void)setFilter:(XLFilter *)filter;

- (XLFilter *)getFilter;
- (void) setImage:(NSString*)item name:(NSString *)name;

- (void)setState:(UIControlState)state value:(float )value;

@end

