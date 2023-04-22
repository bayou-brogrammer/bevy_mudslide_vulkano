use core::fmt;

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::EnumIter;

use crate::utils::U32Visitor;

/// Matter state defines how matter moves
#[repr(u8)]
#[derive(
    EnumIter, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash,
)]
pub enum MatterState {
    Empty = 0,
    Powder = 1,
    Liquid = 2,
    Solid = 3,
    SolidGravity = 4,
    Gas = 5,
}

impl fmt::Display for MatterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

bitflags! {
    /// Reaction cause defines whether a matter causes a reaction
   #[derive(Debug, Copy, Clone)]
   pub struct MatterCharacteristic: u32 {
        /// A material that is corrosive
        const CORROSIVE = 1 << 0;
        /// A material that reacts to corrosive
        const CORRODES = 1 << 1;

        /// A material that can melt others
        const MELTING = 1 << 2;
        /// A material that is melted by melting
        const MELTS = 1 << 3;
   }
}

impl Serialize for MatterCharacteristic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for MatterCharacteristic {
    fn deserialize<D>(deserializer: D) -> Result<MatterCharacteristic, D::Error>
    where
        D: Deserializer<'de>,
    {
        let res = deserializer.deserialize_u32(U32Visitor)?;
        Ok(MatterCharacteristic::from_bits(res).unwrap())
    }
}

pub const ALL_CHARACTERISTICS: [(MatterCharacteristic, &str, &str); 4] = [
    (
        MatterCharacteristic::CORROSIVE,
        "Corrosive",
        "Matter is like acid (destroys other matter)",
    ),
    (
        MatterCharacteristic::CORRODES,
        "Corrodes",
        "Matter is corroded by other matter",
    ),
    (
        MatterCharacteristic::MELTING,
        "Melting",
        "Matter can melt others",
    ),
    (
        MatterCharacteristic::MELTS,
        "Melts",
        "Matter melts by melting matters",
    ),
];
