//
// Created by 周晓林 on 2018/9/5.
//

#ifndef LIBGPUIMAGE_GPUIMAGE_H
#define LIBGPUIMAGE_GPUIMAGE_H

#if __cplusplus
extern "C" {
#endif

long init_context();
void release_context(long context);

long xhey_init_graph();
void release_graph(long graph);


void xhey_picture_graph(long graph, long picture, long basic, long lut, long lut_filter, long unsharpask, long water_mask, long output);

void xhey_graph_forward(long graph);

long xhey_init_alpha_blend(long graph);
void release_alpha_blend_filter(long  filter);

long xhey_init_watermark(long  context);
void release_water_mark_filter(long filter_ptr);
void xhey_watermark_update(long filter,int texId, float x, float y, float w, float h);

long xhey_init_unsharp_mask(long context);
void release_unsharp_mask_filter(long filter);

long xhey_init_basic_filter(long context);
void release_basic_filter(long basic_filter);

long xhey_init_surface_view(long context, int width, int height);
void release_surfaceview(long surface_view);

long xhey_init_picture(void* data, int width, int height);
long xhey_init_picture_textureId(int textureId, int width, int height, int orient);
void release_picture(long picture);

long xhey_init_lookup_filter(long context);
void release_lookup_filter(long lookup);

long xhey_init_picture_output(long context, int width, int height, int orient);
void release_output(long filter);

int xhey_picture_output_get_texture_id(long filter);

#if __cplusplus
}
#endif


#endif //LIBGPUIMAGE_GPUIMAGE_H
