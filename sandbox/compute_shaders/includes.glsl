#include "helpers/index.glsl"

/*
Utility functions to be used in the various kernels:
*/

void write_query_matter(Matter matter) { query_matter[0] = matter_to_uint(matter); }
void write_matter(ivec2 pos, Matter matter) { matter_out[get_index(pos)] = matter_to_uint(matter); }
void write_matter_input(ivec2 pos, Matter matter) { matter_in[get_index(pos)] = matter_to_uint(matter); }
void write_image_color(ivec2 pos, vec4 color) { imageStore(canvas_img, pos, color); }
vec4 matter_color_to_vec4(uint color) {
  return vec4(float((color >> uint(16)) & uint(255)) / 255.0, float((color >> uint(8)) & uint(255)) / 255.0,
              float(color & uint(255)) / 255.0, 1.0);
}
