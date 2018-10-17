//
//  ViewController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2018/9/4.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import "ViewController.h"
#import "gpuimage.h"
#import <objc/runtime.h>
#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "OpenGLView.h"
@interface DemoView : UIView
@end
@implementation DemoView
+ (Class)layerClass{
    return [CAEAGLLayer class];
}




- (id)initWithFrame:(CGRect)frame
{
    self = [super initWithFrame:frame];
    if (self) {

        CAEAGLLayer* _eaglLayer = (CAEAGLLayer*)self.layer;
        
        _eaglLayer.opaque = YES;
        _eaglLayer.drawableProperties = [NSDictionary dictionaryWithObjectsAndKeys:
                                         [NSNumber numberWithBool:NO], kEAGLDrawablePropertyRetainedBacking, kEAGLColorFormatRGBA8, kEAGLDrawablePropertyColorFormat, nil];
        
        
    }
    return self;
}

- (void)dealloc {
    NSLog(@"%s",__func__);
}

@end

@interface ViewController ()
@end

@implementation ViewController




- (void)viewDidLoad {
    [super viewDidLoad];

    self.view.backgroundColor = [UIColor blueColor];
    

    
    DemoView* demoView = [[DemoView alloc] initWithFrame:[UIScreen mainScreen].bounds];
    demoView.center = self.view.center;
    [self.view addSubview:demoView];

    
    
//

//
//
//#if 1
    NSString* path1 = [[NSBundle mainBundle] pathForResource:@"IMG_1592" ofType:@"JPG"];

    UIImage* image1 = [[UIImage alloc] initWithContentsOfFile:path1];
    CGImage* newImageSource1 = [image1 CGImage];
    int width1 = (int)CGImageGetWidth(newImageSource1);
    int height1 = (int)CGImageGetHeight(newImageSource1);

    GLubyte *imageData1 = (GLubyte*)calloc(1, width1*height1*4);
    CGColorSpaceRef genericRGBColorspace1 = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext1 = CGBitmapContextCreate(imageData1, width1, height1, 8, width1*4, genericRGBColorspace1, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext1, CGRectMake(0, 0, width1, height1), newImageSource1);
    CGContextRelease(imageContext1);
    CGColorSpaceRelease(genericRGBColorspace1);
    
    
    XheyTestView* view = xhey_init_test_view((__bridge void*)demoView, imageData1, width1, height1);
    free(imageData1);
    xhey_test_view_display(view);
    
    
    
//    xhey_release_test_view(view);
    
    
    

//
//
//    NSString* path2 = [[NSBundle mainBundle] pathForResource:@"IMG_2333" ofType:@"JPG"];
//
////    UIImage* image2 = [[UIImage alloc] initWithContentsOfFile:path2];
////    CGImage* newImageSource2 = [image2 CGImage];
////    int width2 = (int)CGImageGetWidth(newImageSource2);
////    int height2 = (int)CGImageGetHeight(newImageSource2);
//
////    GLubyte *imageData2 = (GLubyte*)calloc(1, width2*height2*4);
////    CGColorSpaceRef genericRGBColorspace2 = CGColorSpaceCreateDeviceRGB();
////    CGContextRef imageContext2 = CGBitmapContextCreate(imageData2, width2, height2, 8, width2*4, genericRGBColorspace2, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
////    CGContextDrawImage(imageContext2, CGRectMake(0, 0, width2, height2), newImageSource2);
////    CGContextRelease(imageContext2);
////    CGColorSpaceRelease(genericRGBColorspace2);
//
//
//    Graph* graph_ptr = xhey_init_graph();
//
//
//    XheyView* view = xhey_init_view((__bridge void*)demoView);
//    XheyPicture* picture = xhey_init_picture(imageData1, width1, height1);
//    free(imageData1);
//
//    XheyBasicFilter* filter = xhey_init_basic_filter();
//    XheyBasicFilter* filter2 = xhey_init_basic_filter_2();
//
//    xhey_graph(graph_ptr, picture, filter, filter2, view);
//
//
//    xhey_graph_forward(graph_ptr);
//
//    xhey_release_basic_filter(filter2);
//    xhey_release_basic_filter(filter);
//    xhey_release_picture(picture);
//    xhey_release_view(view);
//
//    xhey_context_release();
//    xhey_release_graph(graph_ptr);
//
//    [demoView removeFromSuperview];
//
//


//#endif

    
}



- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
