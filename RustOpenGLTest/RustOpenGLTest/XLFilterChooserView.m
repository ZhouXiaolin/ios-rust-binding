//
//  RDFilterChooserView.m
//  RDVEUISDK
//
//  Created by 周晓林 on 16/4/8.
//
//

#import "XLFilterChooserView.h"
static float const cell_width = 80.0f;

@interface XLFilterChooserView () {
    NSArray *_filters;
    NSArray *_items;
    NSArray *_names;
    NSMutableArray<XLFilterChooserViewCell *> *_cells;
    NSInteger _currentSelectIndex;
}

@end

@implementation XLFilterChooserView


- (void)setCurrentIndex:(NSInteger)currentIndex{
    _currentIndex = currentIndex;
    if ([self ChooserBlock]) {
        _ChooserBlock(currentIndex);
    }
    [_cells enumerateObjectsUsingBlock:^(XLFilterChooserViewCell * _Nonnull obj, NSUInteger idx, BOOL * _Nonnull stop) {
        if(_currentIndex == idx){
            [obj setState:UIControlStateSelected value:1.0];
        }else{
            [obj setState:UIControlStateNormal value:0.0];
        }
    }];
}


- (void) removeItems;
{
    _type  = -1;
    _currentSelectIndex = -1;
    _cells = [NSMutableArray arrayWithCapacity:0];
    
    self.showsHorizontalScrollIndicator = NO;
    
    if (self.subviews.count)
        [self.subviews makeObjectsPerformSelector:@selector(removeFromSuperview)];
    self.contentSize = CGSizeMake(self.frame.size.width, 0);


}
- (void)addItemToChooser:(NSArray *)items itemNames:(NSArray*)names itemPaths:(NSArray*)itemPaths;
{
    if(names.count==0){
        return;
    }
    _type = 1;
    _currentSelectIndex = -1;
    _items = items;
    _names = names;
    _cells = [NSMutableArray arrayWithCapacity:0];
    self.showsHorizontalScrollIndicator = NO;
    if (self.subviews.count)
        [self.subviews makeObjectsPerformSelector:@selector(removeFromSuperview)];
    
    self.contentSize = CGSizeMake(cell_width * _items.count, 0);

    [_items enumerateObjectsUsingBlock:^(NSString*  _Nonnull item, NSUInteger idx, BOOL * _Nonnull stop) {
        XLFilterChooserViewCell *cell = [[XLFilterChooserViewCell alloc] initWithFrame:CGRectMake(self.contentSize.width + idx * cell_width, 0.0f, cell_width, self.bounds.size.height)];
        cell.tag = idx + 1;
        cell.titleLabel.text = names[idx];
        [cell setImage:itemPaths[idx] name:names[idx]];
        UITapGestureRecognizer *tap = [[UITapGestureRecognizer alloc] initWithTarget:self action:@selector(clicked:)];
        [cell addGestureRecognizer:tap];
        [self addSubview:cell];
        [_cells addObject:cell];
    }];
    
    XLFilterChooserViewCell *cell = _cells[_currentIndex];
    [cell setState:UIControlStateSelected value:1.0];

    
}

- (void) addFiltersToChooser: (NSArray<XLFilter*> *)filters{
    if(filters.count==0){
        return;
    }
    _type = 2;
    _currentSelectIndex = -1;
    _filters = filters;
    _cells = [NSMutableArray arrayWithCapacity:0];
    
    self.showsHorizontalScrollIndicator = NO;
    if (self.subviews.count)
        [self.subviews makeObjectsPerformSelector:@selector(removeFromSuperview)];

    self.contentSize = CGSizeMake(cell_width * _filters.count, 0);
    [_filters enumerateObjectsUsingBlock:^(XLFilter *filter, NSUInteger idx, BOOL *stop) {
        XLFilterChooserViewCell *cell = [[XLFilterChooserViewCell alloc] initWithFrame:CGRectMake(self.contentSize.width + idx * cell_width, 0.0f, cell_width, self.bounds.size.height)];
        cell.tag = idx + 1;
        //cell.titleLabel.text = filter.name;
        [cell setFilter:filter];
        UITapGestureRecognizer *tap = [[UITapGestureRecognizer alloc] initWithTarget:self action:@selector(clicked:)];
        [cell addGestureRecognizer:tap];
        [self addSubview:cell];
        [_cells addObject:cell];
    }];
    
    XLFilterChooserViewCell *cell = _cells[_currentIndex];
    [cell setState:UIControlStateSelected value:1.0];
    
}

- (void)layoutSubviews {
    [super layoutSubviews];
    [_cells enumerateObjectsUsingBlock:^(XLFilterChooserViewCell * _Nonnull cell, NSUInteger idx, BOOL * _Nonnull stop) {
        cell.frame = CGRectMake((cell.tag - 1) * cell_width , cell.frame.origin.y, cell.frame.size.width, cell.frame.size.height);

    }];
    
}
- (void)clicked:(UITapGestureRecognizer *)tap {
    if (tap.view.tag == _currentSelectIndex) return;
    

    NSUInteger index = tap.view.tag -1 ;
    
    NSLog(@"%ld",(long)_type);
    XLFilterChooserViewCell *currentcell = (XLFilterChooserViewCell *)tap.view;

    [_cells enumerateObjectsUsingBlock:^(XLFilterChooserViewCell *  _Nonnull cell, NSUInteger idx, BOOL * _Nonnull stop) {
        if (!(idx == tap.view.tag -1)) {
            [cell setState:UIControlStateNormal value:0.0];
        }
    }];
    
    
    _currentSelectIndex = currentcell.tag;
    
    [currentcell setState:UIControlStateSelected value:1.0];
    
    if ([self ChooserBlock]) {
        _ChooserBlock(currentcell.tag - 1);
    }
    
    
   
}

- (void) deleteDownload
{

    [_cells enumerateObjectsUsingBlock:^(XLFilterChooserViewCell * obj, NSUInteger idx, BOOL * _Nonnull stop) {
        [obj removeFromSuperview];
        obj = nil;
    }];
}
- (void)dealloc{
    NSLog(@"%s",__func__);
    
}

@end
