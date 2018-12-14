//
//  RDFilterChooserView.h
//  RDVEUISDK
//
//  Created by 周晓林 on 16/4/8.
//
//

#import <UIKit/UIKit.h>
#import "XLFilterChooserViewCell.h"

//#import <RDRecordGPUImage.h>
@interface XLFilterChooserView : UIScrollView

@property (nonatomic,strong) UIImage* filterImage;
@property (nonatomic,assign,readonly) NSInteger  type;
@property (nonatomic,assign) NSInteger currentIndex;
@property (nonatomic,copy) void(^ChooserBlock) (NSInteger idx);
- (void) removeItems;
- (void) addFiltersToChooser: (NSArray<XLFilter*> *)filters;
- (void) addItemToChooser:(NSArray *)items itemNames:(NSArray*)names itemPaths:(NSArray*)itemPaths;
- (void) deleteDownload;
@end
