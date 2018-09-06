//
// Created by 周晓林 on 2018/9/5.
//

#ifndef LIBGPUIMAGE_GPUIMAGE_H
#define LIBGPUIMAGE_GPUIMAGE_H
#if __cplusplus
extern "C" {
#endif
    struct XheyCamera;
    XheyCamera* xhey_init_camera();
    void xhey_start_capture(XheyCamera* camera);
    void xhey_stop_capture(XheyCamera* camera);


    struct XheyView;
    XheyView* xhey_init_view(void* source);

    struct XheyBasicFilter;
    struct XheyToneCurveFilter;
    struct XHeyLookupTableFilter;
    struct XHeyGaussianBlurFilter;
    void xhey_add_target(void* source, void* consumer);

    int init(void);
#if __cplusplus
}
#endif
#endif //LIBGPUIMAGE_GPUIMAGE_H
