// Get the index of the matter in the matter_in array.
int get_index(ivec2 pos) { return pos.y * sim_canvas_size + pos.x; }

// Get the current position of the thread.
ivec2 get_current_sim_pos() { return ivec2(gl_GlobalInvocationID.xy); }

// Is the current thread inside the simulation canvas?
bool is_inside_sim_canvas(ivec2 pos) {
  return pos.x >= 0 && pos.x < sim_canvas_size && pos.y >= 0 && pos.y < sim_canvas_size;
}

// Get the position of the neighbor in the given direction.
ivec2 get_pos_at_dir(ivec2 pos, int dir) { return pos + OFFSETS[dir]; }

/*
MATTER POSITION QUERIES
*/
Matter read_matter(ivec2 pos) { return new_matter(matter_in[get_index(pos)]); }
bool is_at_border_top(ivec2 pos) { return pos.y == sim_canvas_size - 1; }
bool is_at_border_bottom(ivec2 pos) { return pos.y == 0; }
bool is_at_border_right(ivec2 pos) { return pos.x == sim_canvas_size - 1; }
bool is_at_border_left(ivec2 pos) { return pos.x == 0; }

// | 0 1 2 |
// | 7 x 3 |
// | 6 5 4 |
Matter get_neighbor(ivec2 pos, int dir) {
  ivec2 neighbor_pos = get_pos_at_dir(pos, dir);
  if(is_inside_sim_canvas(neighbor_pos)) {
    return read_matter(neighbor_pos);
  } else {
    return new_matter(empty_matter);
  }
}

/*
MATTER STATE QUERIES
*/
bool is_gas(Matter matter) { return matter.state == state_gas; }
bool is_empty(Matter matter) { return matter.matter == state_empty; }
bool is_powder(Matter matter) { return matter.state == state_powder; }
bool is_liquid(Matter matter) { return matter.state == state_liquid; }
bool is_solid_gravity(Matter matter) { return matter.state == state_solid_gravity; }
bool is_solid(Matter matter) { return matter.state == state_solid || matter.state == state_solid_gravity; }
bool is_gravity(Matter matter) { return is_powder(matter) || is_liquid(matter) || is_solid_gravity(matter); }

/*
MATTER MOVEMENT QUERIES
*/

/*
================== Empty ==================
*/

bool falls_on_empty(Matter from, Matter to) { return is_gravity(from) && is_empty(to); }

/// From could move to both direction to empty, but takes a change at one
/// direction
bool moves_on_empty_maybe(Matter from, Matter to, Matter opposite, Matter down, float p) {
  return p < 0.5 && push_constants.dispersion_step < from.dispersion &&
         ((is_liquid(from) && !is_empty(down)) || is_gas(from)) && is_empty(to) && is_empty(opposite);
}

/// From could move to one direction to empty only
bool moves_on_empty_certainly(Matter from, Matter to, Matter opposite, Matter down) {
  return push_constants.dispersion_step < from.dispersion &&
         ((is_liquid(from) && !is_empty(down)) || is_gas(from)) && is_empty(to) && !is_empty(opposite);
}

/*
For powders
    | |f| |
    |t|x| |
    f->t where x is space under f
*/
bool slides_on_empty(Matter from_diagonal, Matter to_diagonal, Matter from_down) {
  return is_powder(from_diagonal) && !is_empty(from_down) && !is_liquid(from_down) && is_empty(to_diagonal);
}

bool rises_on_empty(Matter from, Matter to) { return is_gas(from) && is_empty(to); }

/*
================== Swap ==================
*/

bool falls_on_swap(Matter from, Matter to) {
  return is_gravity(from) && (is_liquid(to) || is_gas(to)) && to.weight < from.weight;
}

/// From could move in both direction to liquid, but takes a chance at one
/// direction
/// From could move in both direction to liquid, but takes a chance at one
/// direction
bool moves_on_swap_maybe(Matter from, Matter to, Matter opposite, float p) {
  return p < 0.5 && push_constants.dispersion_step < from.dispersion && (is_liquid(from) || is_gas(from)) &&
         (is_liquid(to) || is_gas(to)) && (is_liquid(opposite) || is_gas(opposite)) &&
         opposite.weight < from.weight && to.weight < from.weight;
}

/// From could move to one direction to liquid only
bool moves_on_swap_certainly(Matter from, Matter to, Matter opposite) {
  return push_constants.dispersion_step < from.dispersion && (is_liquid(from) || is_gas(from)) &&
         (is_liquid(to) || is_gas(to)) && !(is_liquid(opposite) && opposite.weight < from.weight) &&
         to.weight < from.weight;
}

bool slides_on_swap(Matter from_diagonal, Matter to_diagonal, Matter from_down) {
  return is_powder(from_diagonal) && !is_empty(from_down) && !is_liquid(from_down) &&
         is_liquid(to_diagonal) && to_diagonal.weight < from_diagonal.weight;
}

bool rises_on_swap(Matter from, Matter to) {
  return is_gas(from) && (is_liquid(to) || is_powder(to)) && to.weight > from.weight;
}