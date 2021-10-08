use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};

type Type = String;

pub struct Statistics {
    pub types: HashMap<Type, TypeStatistic>,
    pub errors: Vec<Error>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            errors: vec![],
        }
    }
}

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
