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
#import "XHImageContext.h"

#import <OpenGLES/ES2/gl.h>
#import <OpenGLES/ES2/glext.h>
#import "OpenGLView.h"
#import "GLESUtils.h"
#import "CameraEntry.h"
#import "MovieWriter.h"
#import "XHFilterController.h"
#import "SecondViewController.h"
@interface ViewController ()
{
    MovieWriter* movieWriter;
    OpenGLView* glView;
    EAGLContext* currentContext;
    
    XHFilterController* filterController;
    
    CameraEntry* cameraEntry;
    
    BOOL isFirst;
    
    
    XLFilterChooserView* filterChooserView;
    
    BOOL isRecording;
    
    
    
}
@end

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    
    
    

    self.view.backgroundColor = [UIColor blueColor];

    int WIDTH = self.view.bounds.size.width;
    int HEIGHT = self.view.bounds.size.height;

    currentContext = [[EAGLContext alloc] initWithAPI:kEAGLRenderingAPIOpenGLES2];




    cameraEntry = [[CameraEntry alloc] initWithSessionPreset:AVCaptureSessionPreset1280x720 location:AVCaptureDevicePositionBack cameraEntryMode:CameraEntryModePhoto4x3 captureAsYUV:TRUE];
//    cameraEntry.isFrontMirrord = YES;
    [cameraEntry addAudioInputsAndOutputs];

    glView = [[OpenGLView alloc] initWithFrame:[UIScreen mainScreen].bounds];
    
    [self.view addSubview:glView];


    filterController = [[XHFilterController alloc]
                        initWithInput:cameraEntry
                        renderView:glView];
    [filterController startCapture];

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
    
//    [filterController stopCapture];
//
//    [filterController clear];
//
//
//    SecondViewController* s = [[SecondViewController alloc] init];
//    [self presentViewController:s animated:FALSE completion:nil];
//    [filterController capturePhotoWithWater:nil
//                         previewImgCallBack:^(UIImage * _Nonnull img, NSError * _Nonnull error)
//    {
//
//    }
//                        originalImgCallBack:^(UIImage * _Nonnull img, NSError * _Nonnull error)
//    {
//
//    }
//                       processedImgCallBack:^(UIImage * _Nonnull img, NSError * _Nonnull error)
//    {
//
//    }];
    
//    [filterController switchCamera];

    
    if (isRecording == NO) {
        isRecording = YES;
        
        [filterController switchCamera];
//        [filterController startRecordWithWaterInfo:nil destinationURL:nil];
    }else{
        isRecording = NO;
        
        [filterController switchCamera];
        
//        [filterController stopRecordWithCompletion:^(NSError * _Nonnull error) {
//        }];

    }
    

}



- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
