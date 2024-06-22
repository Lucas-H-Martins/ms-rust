use std::collections::HashMap;

pub struct AmqpMessage {
    pub msg_type: String,
    pub data: Box<[u8]>,
    pub headers: Option<HashMap<String, String>>,
}

impl AmqpMessage {
    pub fn new<T>(msg_type: T, data: &[u8], headers: Option<HashMap<String, String>>) -> Self
    where
        T: Into<String>,
    {
        AmqpMessage {
            msg_type: msg_type.into(),
            data: data.into(),
            headers,
        }
    }
}
