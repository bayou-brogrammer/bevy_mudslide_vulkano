#version 460

#include "../includes.glsl"

// Rise on empty kernel
void cellular_automata_rise_empty(ivec2 pos) {
  Matter current = read_matter(pos);
  Matter up = get_neighbor(pos, UP);
  Matter down = get_neighbor(pos, DOWN);
  Matter m = current;

  if(!is_at_border_bottom(pos) && rises_on_empty(down, current)) {
    m = down;
  } else if(!is_at_border_top(pos) && rises_on_empty(current, up)) {
    m = up;
  }

  write_matter(pos, m);
}

void main() { cellular_automata_rise_empty(get_current_sim_pos()); }