//
// Created by 周晓林 on 2018/9/5.
//

#ifndef LIBGPUIMAGE_GPUIMAGE_H
#define LIBGPUIMAGE_GPUIMAGE_H

#if __cplusplus
extern "C" {
#endif



    struct XheyPicture;
    struct XheyPicture* xhey_init_picture(void* data, int width, int height);
    void xhey_update_picture(struct XheyPicture* picture, void* data, int width, int height);

    struct XheyView;
    struct XheyView* xhey_init_view(void* source);

    struct XheyBasicFilter;
    struct XheyBasicFilter* xhey_init_basic_filter();
    struct XheyBasicFilter* xhey_init_basic_filter_2();
    
    void xhey_update_outputRotation(void* filter, int rotation);
    
    void xhey_update_outputSize(void* filter, int width, int height);


    struct XheyLookupFilter;
    struct XheyLookupFilter* xhey_init_lookup_filter();

    struct Graph;
    struct Graph * xhey_init_graph();
    void xhey_graph(void* graph, void * source, void* filter,void* filter2, void* consumer);
    void xhey_graph_forward(void*);
    void xhey_graph_printgraphviz(void*);

    struct XHeyCombineFilter;
    struct XHeyCombineFilter* xhey_init_combine_filter();
    void xhey_combine_value(void* , float);


    void xhey_release_picture(void*);
    void xhey_release_view(void*);
    void xhey_release_basic_filter(void*);
    void xhey_release_combine_filter(void*);
    void xhey_context_release();
    void xhey_release_graph(void*);

    struct XheySurfaceView;
    struct XheySurfaceView* xhey_init_surface_view(int width, int height);
    void xhey_release_surface_view(void*);

    struct XheyPictureOutput;
    struct XheyPictureOutput* xhey_init_picture_output(int width, int height);
    unsigned int xhey_picture_output_get_texture_id(void*);
#if __cplusplus
}
#endif


#endif //LIBGPUIMAGE_GPUIMAGE_H
