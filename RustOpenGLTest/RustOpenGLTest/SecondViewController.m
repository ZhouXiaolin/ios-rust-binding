//
//  SecondViewController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2019/2/16.
//  Copyright © 2019 Solaren. All rights reserved.
//

#import "SecondViewController.h"
#import "OpenGLView.h"
#import "XLHelpClass.h"

#import "XHFilterController.h"
#import "XLFilterChooserView.h"
@interface SecondViewController ()
{
    OpenGLView* glView;
    XHFilterController* filterController;
    EAGLContext* currentContext;
    XLFilterChooserView* filterChooserView;


}

@end

@implementation SecondViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    
    self.view.backgroundColor = [UIColor redColor];
    
    
    self.view.backgroundColor = [UIColor blueColor];
    
    int WIDTH = self.view.bounds.size.width;
    int HEIGHT = self.view.bounds.size.height;
    
    
    glView = [[OpenGLView alloc] initWithFrame:[UIScreen mainScreen].bounds context:currentContext];
    [self.view addSubview:glView];
    
    filterController = [[XHFilterController alloc]
                        initWithPicture:[UIImage imageNamed:@"aaa"] renderView:glView];
    [filterController renderPictureWithLut:nil];
    
    
    NSString* bundlePath = [XLHelpClass pathBundlePath];
    NSArray<NSString*>* files = [[NSFileManager defaultManager] subpathsAtPath:bundlePath];
    
    filterChooserView = [[XLFilterChooserView alloc] initWithFrame:CGRectMake(0, HEIGHT - 80, WIDTH, 80)];
    filterChooserView.backgroundColor = UIColorFromRGB(0x19181d);
    [filterChooserView setChooserBlock:^(NSInteger idx) {
        
        NSString* name = files[idx];
        NSString* path = [bundlePath stringByAppendingFormat:@"/%@",name];
        
        
        if (idx > 0) {
            [filterController changeLookup:path];
        }
        
    }];
    
    [self.view addSubview:filterChooserView];
    
    NSMutableArray<XLFilter*>* array = [NSMutableArray array];
    for (NSString* path in files) {
        XLFilter* filter = [[XLFilter alloc] init];
        filter.name = path;
        [array addObject:filter];
    }
    
    [filterChooserView addFiltersToChooser:array];
    [filterChooserView setCurrentIndex:0];
    
    UIButton* button = [UIButton buttonWithType:UIButtonTypeCustom];
    button.frame = CGRectMake(0, 0, 50, 50);
    button.center = self.view.center;
    button.backgroundColor = [UIColor redColor];
    [button addTarget:self action:@selector(click) forControlEvents:UIControlEventTouchUpInside];
    [self.view addSubview:button];
    
    
    
}
- (void) click
{
    
    
}

/*
#pragma mark - Navigation

// In a storyboard-based application, you will often want to do a little preparation before navigation
- (void)prepareForSegue:(UIStoryboardSegue *)segue sender:(id)sender {
    // Get the new view controller using [segue destinationViewController].
    // Pass the selected object to the new view controller.
}
*/

@end
