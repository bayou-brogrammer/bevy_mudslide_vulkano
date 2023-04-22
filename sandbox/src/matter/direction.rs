use std::fmt;

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::utils::U32Visitor;

bitflags! {
    /// Reaction cause defines whether a matter causes a reaction
    #[derive(Debug, Copy, Clone)]
    pub struct Direction: u32 {
        const NONE = 0;
        const UP_LEFT = 1 << 0;
        const UP = 1 << 1;
        const UP_RIGHT = 1 << 2;
        const RIGHT = 1 << 3;
        const DOWN_RIGHT = 1 << 4;
        const DOWN = 1 << 5;
        const DOWN_LEFT = 1 << 6;
        const LEFT = 1 << 7;
        // const ALL = 0b11111111;
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::str::FromStr for Direction {
    type Err = bitflags::parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        Ok(Self(flags.parse()?))
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Direction, D::Error>
    where
        D: Deserializer<'de>,
    {
        let res = deserializer.deserialize_u32(U32Visitor)?;
        Ok(Direction::from_bits(res).unwrap())
    }
}

pub const ALL_DIRECTIONS: [(Direction, &str); 8] = [
    (Direction::UP, "Up"),
    (Direction::DOWN, "Down"),
    (Direction::LEFT, "Left"),
    (Direction::RIGHT, "Right"),
    (Direction::UP_LEFT, "Up Left"),
    (Direction::UP_RIGHT, "Up Right"),
    (Direction::DOWN_LEFT, "Down Left"),
    (Direction::DOWN_RIGHT, "Down Right"),
];
