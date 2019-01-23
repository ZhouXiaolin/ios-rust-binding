//
//  ViewController.m
//  RustOpenGLTest
//
//  Created by 周晓林 on 2018/9/4.
//  Copyright © 2018年 Solaren. All rights reserved.
//

#import "ViewController.h"
#import "gpuimage.h"
#import "XLHelpClass.h"
#import "XLFilterChooserView.h"
#import <objc/runtime.h>

#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "OpenGLView.h"
#import "GLESUtils.h"
#import "CameraEntry.h"
#import "MovieWriter.h"
#import "FilterController.h"

@interface ViewController ()
{
    MovieWriter* movieWriter;
    OpenGLView* glView;
    EAGLContext* currentContext;
    
    FilterController* filterController;
    
    CameraEntry* cameraEntry;
    
    BOOL isFirst;
    
    NSString* lut_path;
    BOOL update;
    
    XLFilterChooserView* filterChooserView;
    
    BOOL isRecording;
    
    
    
}
@end

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    
    
    movieWriter = [[MovieWriter alloc] init];
    

    self.view.backgroundColor = [UIColor blueColor];
    
    int WIDTH = self.view.bounds.size.width;
    int HEIGHT = self.view.bounds.size.height;
    
    currentContext = [[EAGLContext alloc] initWithAPI:kEAGLRenderingAPIOpenGLES2];
    

    
    cameraEntry = [[CameraEntry alloc] initWithSessionPreset:(AVCaptureSessionPreset1280x720) location:(AVCaptureDevicePositionBack) captureAsYUV:TRUE];
    
    
    glView = [[OpenGLView alloc] initWithFrame:[UIScreen mainScreen].bounds context:currentContext];
    [self.view addSubview:glView];
    
    filterController = [[FilterController alloc]
                        initWithInput:cameraEntry
                        renderView:glView
                        writer:movieWriter
                        context:currentContext];
    [filterController startCapture];

    NSString* bundlePath = [XLHelpClass pathBundlePath];
    NSArray<NSString*>* files = [[NSFileManager defaultManager] subpathsAtPath:bundlePath];
    
    filterChooserView = [[XLFilterChooserView alloc] initWithFrame:CGRectMake(0, HEIGHT - 80, WIDTH, 80)];
    filterChooserView.backgroundColor = UIColorFromRGB(0x19181d);
    [filterChooserView setChooserBlock:^(NSInteger idx) {
        
        NSString* name = files[idx];
        NSString* path = [bundlePath stringByAppendingFormat:@"/%@",name];
        
        NSLog(@"solaren %@",path);
        self->update = YES;
        
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
    
    if (isRecording == NO) {
        isRecording = YES;
        [filterController startRecordWithWaterInfo:nil destinationURL:nil];
    }else{
        isRecording = NO;
        [filterController stopRecordWithCompletion:^(NSError * _Nonnull error) {
            
        }];
       
    }
    
}



- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
