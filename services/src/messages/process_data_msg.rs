use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::amqp::AmqpError;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProcessDataTimer {
    pub id: i32,
    pub ttl: i32,
    pub message: String,
}

impl TryFrom<&[u8]> for ProcessDataTimer {
    type Error = AmqpError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match serde_json::from_slice(value) {
            Ok(r) => Ok(r),
            _ => Err(AmqpError::UnpackMessageError),
        }
    }
}

impl Display for ProcessDataTimer {
    fn fmt(&self, value: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(value, "ProcessDataTimer")
    }
}
