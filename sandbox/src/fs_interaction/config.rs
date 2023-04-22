use bevy::{prelude::*, reflect::TypeUuid};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Reflect,
    FromReflect,
    TypeUuid,
    Serialize,
    Deserialize,
    Default,
    Resource,
)]
#[reflect(Serialize, Deserialize, Resource)]
#[uuid = "93a7c64b-4d6e-4420-b8c1-dfca481d9387"]
pub struct GameConfig {
    pub definition_path: Option<String>,
}
