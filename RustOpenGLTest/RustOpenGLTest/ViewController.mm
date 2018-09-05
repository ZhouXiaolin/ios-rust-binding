//
//  ViewController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2018/9/4.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import "ViewController.h"
#import "OpenGLView.h"
#import "gpuimage.h"

@interface DemoView : UIView
@end
@implementation DemoView
+ (Class)layerClass{
    return [CAEAGLLayer class];
}
@end

@interface ViewController ()
{
    CAEAGLLayer* _eaglLayer;
}
@end

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];

    
    DemoView* demoView = [[DemoView alloc] initWithFrame:CGRectMake(0, 0, 100, 100)];
    demoView.center = self.view.center;
    [self.view addSubview:demoView];
    

    
    
    
    XheyView* view = nullptr;
    XheyCamera* camera = nullptr;
    xhey_init_camera(camera);

    xhey_init_view(view,(__bridge void*)demoView);
    xhey_add_target(camera, view);

    xhey_start_capture(camera);
    
    
    
//    OpenGLView* view =[[OpenGLView alloc] initWithFrame:CGRectMake(0, 0, 200, 200)];
//    view.center = self.view.center;
//    [self.view addSubview:view];
    
    // Do any additional setup after loading the view, typically from a nib.
}


- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
