pub mod direction;
pub mod matter_definition;
pub mod matter_reaction;
pub mod matter_state;

use bevy::{prelude::Resource, utils::HashMap};

use self::{
    matter_definition::{MatterDefinition, MatterDefinitions},
    matter_reaction::MatterReaction,
    matter_state::{MatterCharacteristic, MatterState},
};
use crate::utils::u8_rgba_to_u32_rgba;

#[derive(Resource)]
pub struct Matters(pub HashMap<u32, String>);

/// Matter data where first 3 bytes are saved for color and last 4th byte is saved for matter
/// identifier
#[derive(Default, Copy, Clone)]
pub struct MatterWithColor {
    pub value: u32,
}

impl MatterWithColor {
    /// Creates a new matter with color from matter id giving it a slightly randomized color
    pub fn new(matter_id: u32, color: [u8; 4]) -> MatterWithColor {
        MatterWithColor {
            value: u8_rgba_to_u32_rgba(color[0], color[1], color[2], matter_id as u8),
        }
    }

    pub fn matter_id(&self) -> u32 {
        self.value & 255
    }
}

impl From<u32> for MatterWithColor {
    fn from(item: u32) -> Self {
        Self { value: item }
    }
}

pub const MAX_NUM_MATTERS: u32 = 256;
pub const MATTER_DEFINITION_FILE: &str = "assets/matter_definitions.matter.ron";

pub const MATTER_EMPTY: u32 = 0;
pub const MATTER_SAND: u32 = 1;
pub const MATTER_WATER: u32 = 2;
pub const MATTER_GAS: u32 = 3;

pub fn default_matter_definitions() -> MatterDefinitions {
    MatterDefinitions {
        empty: MATTER_EMPTY,
        definitions: vec![
            MatterDefinition {
                id: MATTER_EMPTY,
                weight: 0.0,
                dispersion: 0,
                color: 0x0,
                name: "Empty".to_string(),
                state: MatterState::Empty,
                reactions: MatterReaction::all_zero(),
                characteristics: MatterCharacteristic::empty(),
            },
            MatterDefinition {
                id: MATTER_SAND,
                weight: 1.5,
                dispersion: 0,
                color: 0xc2b280ff,
                name: "Sand".to_string(),
                state: MatterState::Powder,
                reactions: MatterReaction::all_zero(),
                characteristics: (MatterCharacteristic::MELTS | MatterCharacteristic::CORRODES),
            },
            MatterDefinition {
                id: MATTER_WATER,
                weight: 1.0,
                dispersion: 10,
                color: 0x1ca3ecff,
                name: "Water".to_string(),
                state: MatterState::Liquid,
                reactions: MatterReaction::all_zero(),
                characteristics: MatterCharacteristic::empty(),
            },
            MatterDefinition {
                id: MATTER_GAS,
                weight: 0.1,
                dispersion: 5,
                color: 0x92cd00ff,
                name: "Gas".to_string(),
                state: MatterState::Gas,
                reactions: MatterReaction::all_zero(),
                ..MatterDefinition::zero()
            },
        ],
    }
}
