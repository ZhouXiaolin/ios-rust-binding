//
// Created by 周晓林 on 2018/9/5.
//

#ifndef LIBGPUIMAGE_GPUIMAGE_H
#define LIBGPUIMAGE_GPUIMAGE_H

#if __cplusplus
extern "C" {
#endif


typedef void (*hook)(void* context);


long init_context();
void release_context(long context);

long xhey_init_graph();
void release_graph(long graph);


enum InputKind{
    Picture,
    Camera
};

enum OutputKind{
    AlphaBlend,
    Basic,
    Blend,
    Combine,
    Lookup,
    UnsharpMask,
    PictureOutput
};

long xhey_graph_add_input(long graph, long filter,InputKind kind);
long xhey_graph_add_function(long graph, long filter, long* arg,OutputKind kind);


void xhey_camera_graph(long graph, long camera, long basic, long basic_normal,long lut, long lut_filter, long tone_curve, long tone_curve_filter,long unsharpask, long water_mask, long output, long normal_output);
void xhey_picture_graph(long graph, long picture, long basic, long lut, long lut_filter, long unsharpask, long water_mask, long output);
void xhey_camera_watermark_graph(long graph, long picture, long  basic, long water_mark, long  water_output);
void xhey_normal_camera_graph(long graph, long camera, long basic, long output);
void xhey_graph_forward(long graph);
void xhey_print_graph(long graph);

long xhey_init_alpha_blend(long graph);
void release_alpha_blend_filter(long  filter);

long xhey_init_watermark(long  context);
void release_water_mark_filter(long filter_ptr);
void xhey_watermark_update(long filter,int texId, float x, float y, float w, float h,int rotation);

long xhey_init_unsharp_mask(long context);
void release_unsharp_mask_filter(long filter);

long xhey_init_basic_filter(long context);
long xhey_update_basic_hook(long basic_filter, hook, void*);
void release_basic_filter(long basic_filter);
void xhey_update_basic_rotation(long basic_filter, int);
int xhey_get_basic_texture_id(long basic_filter);
void xhey_update_basic_size(long basic_filter, int, int);

long xhey_init_surface_view(long context, int width, int height);
void release_surfaceview(long surface_view);

long xhey_init_picture(void* data, int width, int height);
long xhey_init_picture_textureId(int textureId, int width, int height, int orient);
void xhey_picture_update(long picture, void* data, int width, int height);
void release_picture(long picture);

long xhey_init_lookup_filter(long context);
void release_lookup_filter(long lookup);
void xhey_update_lookup_intensity(long lut, float v);

long xhey_init_picture_output(long context, int width, int height, int orient);
void xhey_update_picture_output_hook(long pic_output, hook,void*);
void release_output(long filter);
int xhey_picture_output_get_texture_id(long filter);
void xhey_update_output_size(long pic_output, int width, int height);
void xhey_update_output_rotation(long pic_output, int rotation);

long xhey_init_tone_curve(long context);

long xhey_init_camera(long context, int width, int height, int orient);
void camera_update_luminance(long filter, int luminance);
void camera_update_chrominance(long filter, int chrominance);
void camera_update_matrix(long filter, float* mat);
void camera_update_size(long filter, int width, int height);
void camera_update_rotation(long filter, int rotation);
void release_camera(long filter);
#if __cplusplus
}
#endif


#endif //LIBGPUIMAGE_GPUIMAGE_H
