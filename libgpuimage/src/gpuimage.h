//
// Created by 周晓林 on 2018/9/5.
//

#ifndef LIBGPUIMAGE_GPUIMAGE_H
#define LIBGPUIMAGE_GPUIMAGE_H
#if __cplusplus
extern "C" {
#endif


    struct XheyCamera;
    struct XheyCamera* xhey_init_camera(void);
    void xhey_start_capture(struct XheyCamera* camera);
    void xhey_stop_capture(struct XheyCamera* camera);

    struct XheyPicture;
    struct XheyPicture* xhey_init_picture(void* data, int width, int height);
    void xhey_process_picture(struct XheyPicture* picture);

    struct XheyView;
    struct XheyView* xhey_init_view(void* source);

    struct XheyBasicFilter;
    struct XheyBasicFilter* xhey_init_basic_filter();
    struct XheyBasicFilter* xhey_init_basic_filter_2();

    struct XheyToneCurveFilter;
    struct XHeyLookupTableFilter;
    struct XHeyGaussianBlurFilter;

    struct Graph;
    struct Graph * xhey_init_graph();
    void xhey_graph(void* graph, void * source, void* filter,void* filter2, void* filter3,void* consumer);
    void xhey_graph_forward(void*);
    void xhey_graph_printgraphviz(void*);

#if __cplusplus
}
#endif
#endif //LIBGPUIMAGE_GPUIMAGE_H
