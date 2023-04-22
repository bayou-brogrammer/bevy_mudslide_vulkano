use std::fmt;

use serde::de::{self, Visitor};

pub struct U32Visitor;
impl<'de> Visitor<'de> for U32Visitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 2^32")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(u32::from(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value >= u64::from(u32::MIN) && value <= u64::from(u32::MAX) {
            Ok(value as u32)
        } else {
            Err(E::custom(format!("u32 out of range: {}", value)))
        }
    }
}
