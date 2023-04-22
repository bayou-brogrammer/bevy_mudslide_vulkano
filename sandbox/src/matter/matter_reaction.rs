use serde::{Deserialize, Serialize};

use super::{direction::Direction, matter_state::MatterCharacteristic};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct MatterReaction {
    pub becomes: u32,
    pub probability: f32,
    pub direction: Direction,
    pub reacts: MatterCharacteristic,
}

impl MatterReaction {
    pub fn all_zero() -> [Self; 5] {
        [
            MatterReaction::zero(),
            MatterReaction::zero(),
            MatterReaction::zero(),
            MatterReaction::zero(),
            MatterReaction::zero(),
        ]
    }

    pub fn zero() -> Self {
        MatterReaction {
            becomes: 0,
            probability: 0.0,
            direction: Direction::NONE,
            reacts: MatterCharacteristic::empty(),
        }
    }

    pub fn dies(p: f32, empty_matter: u32) -> Self {
        MatterReaction {
            probability: p,
            becomes: empty_matter,
            direction: Direction::all(),
            reacts: MatterCharacteristic::empty(),
        }
    }

    pub fn becomes_on_touch(
        p: f32,
        touch_characteristic: MatterCharacteristic,
        becomes_matter: u32,
    ) -> Self {
        MatterReaction {
            probability: p,
            becomes: becomes_matter,
            direction: Direction::all(),
            reacts: touch_characteristic,
        }
    }

    // Good for e.g. fire
    pub fn becomes_on_touch_below(
        p: f32,
        touch_characteristic: MatterCharacteristic,
        becomes_matter: u32,
    ) -> Self {
        MatterReaction {
            probability: p,
            becomes: becomes_matter,
            reacts: touch_characteristic,
            direction: (Direction::DOWN
                | Direction::DOWN_LEFT
                | Direction::DOWN_RIGHT
                | Direction::RIGHT
                | Direction::LEFT),
        }
    }
}
