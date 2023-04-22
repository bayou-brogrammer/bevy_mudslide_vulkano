use bevy::prelude::IVec2;

use crate::SIM_CANVAS_SIZE;

// /// Index to access our one dimensional grid with two dimensional position
pub fn idx(pos: IVec2) -> usize {
    (pos.y * SIM_CANVAS_SIZE as i32 + pos.x) as usize
}

pub fn is_inside_sim_canvas(pos: IVec2) -> bool {
    pos.x >= 0 && pos.x < SIM_CANVAS_SIZE as i32 && pos.y >= 0 && pos.y < SIM_CANVAS_SIZE as i32
}
