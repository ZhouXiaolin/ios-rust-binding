//
//  RDFilterChooserViewCell.m
//  RDVEUISDK
//
//  Created by 周晓林 on 16/4/8.
//
//


#import "XLFilterChooserViewCell.h"
#import "XLHelpClass.h"
@implementation XLFilter

@end
@interface XLFilterChooserViewCell () {
    UIImageView *view1;
    XLFilter * group;
}
@end

@implementation XLFilterChooserViewCell



- (instancetype)initWithFrame:(CGRect)frame
{
    self = [super initWithFrame:frame];
    if (self) {
        
        
        view1 = [[UIImageView alloc] initWithFrame:CGRectMake(0, 0, 56, 56)];
        view1.center = CGPointMake(40, 30);
        view1.layer.masksToBounds = YES;
        view1.layer.cornerRadius = 28;

        
        [self addSubview:view1];
        [self addSubview:self.titleLabel];
    }
    return self;
}
- (void) setImage:(NSString*)item name:(NSString *)name
{
    
    UIImage* inputImage;
   
    
    inputImage = [UIImage imageWithContentsOfFile:[[NSBundle mainBundle].resourcePath stringByAppendingPathComponent:[NSString stringWithFormat:@"VideoRecord.bundle/faceunity/%@.png",item]]];
    if (inputImage) {
        [view1 setImage:inputImage];
    }else{
        view1.backgroundColor = [UIColor grayColor];
    }

    
}
- (void)setFilter:(XLFilter *)filter {
    
//    UIImage* image = [UIImage imageWithContentsOfFile:[[NSBundle mainBundle] pathForResource:@"f_positano_01" ofType:@"png"]];
    UIImage* image = [UIImage imageNamed:@"aaa"];
    
    
    
    [view1 setImage:image];
    _titleLabel.text = filter.name;

    
    //group = filter;
    view1.layer.borderColor = [UIColor colorWithRed:40.0/255.0 green:202.0/255.0 blue:217.0/255.0 alpha:1.0].CGColor;
}

- (UILabel *)titleLabel {
    if (!_titleLabel) {
        UILabel *titleLabel = [[UILabel alloc] initWithFrame:CGRectMake(10, 62, view1.bounds.size.width, 20)];
        titleLabel.textAlignment = NSTextAlignmentCenter;
        titleLabel.textColor = [UIColor grayColor];
        titleLabel.font = [UIFont systemFontOfSize:12];
        titleLabel.adjustsFontSizeToFitWidth = YES;
        _titleLabel = titleLabel;
    }
    return _titleLabel;
}

- (UIImageView *)backgroudView{
    if (!_backgroudView) {
        UIImageView* imageView = [[UIImageView alloc] initWithFrame:CGRectMake(0, 0, 60, 60)];
        imageView.center = CGPointMake(40, 30);
        
        _backgroudView = imageView;
    }
    return _backgroudView;
}
- (XLFilter *)getFilter {
    
    return group;
    
}

- (void)setState:(UIControlState)state value:(float )value
{
    
    switch (state) {
        case UIControlStateNormal: {
            [_titleLabel setTextColor:[UIColor whiteColor]];
        }
            break;
        case UIControlStateSelected:{
            [_titleLabel setTextColor:UIColorFromRGB(0xffd500)];
        }
            break;
        default:
            break;
    }
    
}

@end
