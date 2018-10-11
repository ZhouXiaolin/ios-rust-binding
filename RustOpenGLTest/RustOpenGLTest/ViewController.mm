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



@end

@interface ViewController ()
@end

@implementation ViewController



void initImageData(UIImage* image, GLubyte* imageData, int* width, int* height){
    CGImage* newImageSource = [image CGImage];
    *width = (int)CGImageGetWidth(newImageSource);
    *height = (int)CGImageGetHeight(newImageSource);
 
    imageData = (GLubyte*)calloc(1, (*width) * (*height) * 4);
    CGColorSpaceRef genericRGBColorspace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext = CGBitmapContextCreate(imageData, *width, *height, 8, (*width) * 4, genericRGBColorspace, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext, CGRectMake(0, 0, *width, *height), newImageSource);
    CGContextRelease(imageContext);
    CGColorSpaceRelease(genericRGBColorspace);
}

- (void)viewDidLoad {
    [super viewDidLoad];

    self.view.backgroundColor = [UIColor blueColor];
    
    
    DemoView* demoView = [[DemoView alloc] initWithFrame:[UIScreen mainScreen].bounds];
    demoView.center = self.view.center;
    [self.view addSubview:demoView];

//
    
    
    
#if 1
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
    
    
    NSString* path2 = [[NSBundle mainBundle] pathForResource:@"IMG_2333" ofType:@"JPG"];
    
    UIImage* image2 = [[UIImage alloc] initWithContentsOfFile:path2];
    CGImage* newImageSource2 = [image2 CGImage];
    int width2 = (int)CGImageGetWidth(newImageSource2);
    int height2 = (int)CGImageGetHeight(newImageSource2);
    
    GLubyte *imageData2 = (GLubyte*)calloc(1, width2*height2*4);
    CGColorSpaceRef genericRGBColorspace2 = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext2 = CGBitmapContextCreate(imageData2, width2, height2, 8, width2*4, genericRGBColorspace2, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext2, CGRectMake(0, 0, width2, height2), newImageSource2);
    CGContextRelease(imageContext2);
    CGColorSpaceRelease(genericRGBColorspace2);



    Graph* graph = xhey_init_graph();
//
    XheyView* view = xhey_init_view((__bridge void*)demoView);
    XheyPicture* picture = xhey_init_picture(imageData1, width1, height1);
    XheyPicture* lookup_pic = xhey_init_picture(imageData2, width2, height2);
    
    XheyLookupFilter* filter = xhey_init_lookup_filter();
    

    xhey_graph(graph, picture, lookup_pic, filter, view);


    xhey_graph_printgraphviz(graph);

    



    __block float value = 0.9;
    [NSTimer scheduledTimerWithTimeInterval:0.3 repeats:YES block:^(NSTimer * _Nonnull timer) {

        xhey_graph_forward(graph);

//        if (value > 1.0) {
//            value = 0.3;
//            xhey_update_picture(picture, imageData2, width2, height2);
//        }else{
//            value += 0.1;
//            xhey_update_picture(picture, imageData1, width1, height1);
//
//        }


    }];

#endif
    
    
}



- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
