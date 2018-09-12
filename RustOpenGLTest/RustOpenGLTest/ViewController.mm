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


- (void)viewDidLoad {
    [super viewDidLoad];

    self.view.backgroundColor = [UIColor blueColor];
    

    
    
    DemoView* demoView = [[DemoView alloc] initWithFrame:[UIScreen mainScreen].bounds];
    demoView.center = self.view.center;
    [self.view addSubview:demoView];
////
////
////
//////    const char* path = [@"IMG_1592" UTF8String];
//////    UIImage* image = (__bridge UIImage*)test(path);
//
    UIImage* image = [UIImage imageNamed:@"IMG_1592"];
    CGImage* newImageSource = [image CGImage];
    int width = (int)CGImageGetWidth(newImageSource);
    int height = (int)CGImageGetHeight(newImageSource);

    GLubyte *imageData = (GLubyte*)calloc(1, width*height*4);
    CGColorSpaceRef genericRGBColorspace = CGColorSpaceCreateDeviceRGB();
    CGContextRef imageContext = CGBitmapContextCreate(imageData, width, height, 8, width*4, genericRGBColorspace, kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
    CGContextDrawImage(imageContext, CGRectMake(0, 0, width, height), newImageSource);
    CGContextRelease(imageContext);
    CGColorSpaceRelease(genericRGBColorspace);

    XheyView* view = xhey_init_view((__bridge void*)demoView,(const void*)imageData, width, height);

    XheyPicture* picture = xhey_init_picture();
    
    xhey_add_target(picture, view);
    
    xhey_process_picture(picture);
//
    free(imageData);
    
    // Do any additional setup after loading the view, typically from a nib.
}


- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
