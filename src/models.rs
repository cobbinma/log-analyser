use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom};

#[derive(Debug, Clone)]
pub struct TypeStatistic {
    pub total_messages: u64,
    pub total_byte_size: u64,
}

impl TypeStatistic {
    pub fn add_message(&mut self, message: &Message) {
        self.total_messages += 1;
        self.total_byte_size += message.byte_size;
    }
}

#[derive(Debug)]
pub struct Message {
    pub type_field: String,
    pub byte_size: u64,
}

impl Message {
    pub fn error() -> Self {
        Message {
            type_field: "errors".to_string(),
            byte_size: 0,
        }
    }
}

impl TryFrom<String> for Message {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Error> {
        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct LogLine {
            #[serde(rename = "type")]
            pub type_field: String,
        }
        let LogLine { type_field } = serde_json::from_str(&value).map_err(Error::from)?;

        Ok(Message {
            type_field,
            byte_size: value.as_bytes().len() as u64,
        })
    }
}
