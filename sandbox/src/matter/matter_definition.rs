use bevy::{prelude::Resource, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

use super::{
    matter_reaction::MatterReaction,
    matter_state::{MatterCharacteristic, MatterState},
};

/// If you touch this, also change shaders...
pub const MAX_TRANSITIONS: u32 = 5;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatterDefinition {
    pub id: u32,
    pub color: u32,
    pub weight: f32,
    pub name: String,
    pub dispersion: u32,
    pub state: MatterState,

    /// What are the characteristics of matter?
    /// - Water: "Cools", "Rusts"
    /// - Acid: "Corrodes".
    /// Think of it like: "What does this do to others?"
    pub characteristics: MatterCharacteristic,

    /// How does matter react to neighbor characteristics?
    /// - Example: "Water becomes ice on probability x if touches one that freezes".
    /// - Example: "Acid might become empty on probability x if touches a material it corroded
    ///   (corroding)".
    /// Probability will affect the speed at which matter changes
    pub reactions: [MatterReaction; MAX_TRANSITIONS as usize],
}

impl Default for MatterDefinition {
    fn default() -> Self {
        Self::zero()
    }
}

impl MatterDefinition {
    pub fn zero() -> Self {
        MatterDefinition {
            id: 0,
            color: 0x0,
            weight: 0.0,
            dispersion: 0,
            state: MatterState::Empty,
            name: "Empty".to_string(),
            characteristics: MatterCharacteristic::empty(),
            reactions: [
                MatterReaction::zero(),
                MatterReaction::zero(),
                MatterReaction::zero(),
                MatterReaction::zero(),
                MatterReaction::zero(),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Resource, TypeUuid)]
#[uuid = "f3b0c0f0-1d1a-4b0a-9b0a-1d1a4b0a9b0a"]
pub struct MatterDefinitions {
    pub empty: u32,
    pub definitions: Vec<MatterDefinition>,
}

impl MatterDefinitions {
    pub fn serialize(&self) -> String {
        ron::ser::to_string_pretty(
            self,
            ron::ser::PrettyConfig::new()
                .struct_names(true)
                .enumerate_arrays(true)
                .separate_tuple_members(true),
        )
        .unwrap()
    }
}

pub fn validate_matter_definitions(matter_definitions: &MatterDefinitions) {
    for (i, m) in matter_definitions.definitions.iter().enumerate() {
        if m.id != i as u32 {
            panic!(
                "Invalid matter definition, definition {}: id {} does not equal matter id index {}",
                m.name,
                { m.id },
                i as u32
            );
        }

        if m.reactions
            .iter()
            .any(|r| r.becomes >= matter_definitions.definitions.len() as u32)
        {
            panic!(
                "Matter reaction invalid for id: {}, name: {}. 'becomes' must not be larger than \
                 any id",
                m.id, m.name
            )
        }
    }
}
