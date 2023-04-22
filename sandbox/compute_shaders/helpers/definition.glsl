/*
Specialization constants
*/
layout(constant_id = 0) const uint empty_matter = 1;
layout(constant_id = 1) const int sim_canvas_size = 1;
layout(constant_id = 2) const uint state_empty = 1;
layout(constant_id = 3) const uint state_powder = 1;
layout(constant_id = 4) const uint state_liquid = 1;
layout(constant_id = 5) const uint state_solid = 1;
layout(constant_id = 6) const uint state_solid_gravity = 1;
layout(constant_id = 7) const uint state_gas = 1;
layout(local_size_x_id = 8, local_size_y_id = 9, local_size_z = 1) in;

/*
Buffers
*/
layout(set = 0, binding = 0) restrict buffer MatterStateBuffer { uint matter_state[]; };
layout(set = 0, binding = 1) restrict buffer MatterWeightsBuffer { float matter_weights[]; };
layout(set = 0, binding = 2) restrict buffer MatterDispersionBuffer { uint matter_dispersion[]; };
/*
Matter data chunks
*/
layout(set = 0, binding = 3) restrict buffer MatterInBuffer { uint matter_in[]; };
layout(set = 0, binding = 4) restrict writeonly buffer MatterOutBuffer { uint matter_out[]; };
layout(set = 0, binding = 5) restrict writeonly buffer QueryMatterBuffer { uint query_matter[]; };
layout(set = 0, binding = 6, rgba8) restrict uniform writeonly image2D canvas_img;

layout(push_constant) uniform PushConstants {
  float seed;
  uint sim_steps;
  vec2 draw_pos_start;
  vec2 draw_pos_end;
  float draw_radius;
  uint draw_matter;
  ivec2 query_pos;
  uint dispersion_dir;
  uint move_step;
  uint dispersion_step;
  bool is_square;
}
push_constants;