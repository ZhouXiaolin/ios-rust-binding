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
    
    EAGLContext* context = [EAGLContext currentContext];
    context = nil;
    [EAGLContext setCurrentContext:nil];
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


    
    Graph* g = xhey_init_graph();
    XheyPicture* pic = xhey_init_picture(imageData1, width1, height1);
    free(imageData1);
    XheyView* view = xhey_init_view((__bridge void*)demoView);
    xhey_graph(g, pic, nullptr, nullptr, view);
    
    xhey_graph_forward(g);

    

    
}



- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
