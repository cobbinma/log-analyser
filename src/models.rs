use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};

type Type = String;

pub struct Statistics {
    types: HashMap<Type, TypeStatistic>,
    errors: Vec<Error>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            errors: vec![],
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.types
            .entry(message.type_field.clone())
            .or_insert(TypeStatistic {
                total_messages: 0,
                total_byte_size: 0,
            })
            .add_message(&message)
    }

    pub fn add_error(&mut self, error: Error) {
        self.errors.push(error)
    }

    pub fn types(&self) -> &HashMap<Type, TypeStatistic> {
        &self.types
    }

    pub fn errors(&self) -> &Vec<Error> {
        &self.errors
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeStatistic {
    pub total_messages: u64,
    pub total_byte_size: u64,
}

impl TypeStatistic {
    fn add_message(&mut self, message: &Message) {
        self.total_messages += 1;
        self.total_byte_size += message.byte_size;
    }
}

#[derive(Debug)]
pub struct Message {
    pub type_field: Type,
    pub byte_size: u64,
}

impl TryFrom<String> for Message {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Error> {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct LogLine {
            #[serde(rename = "type")]
            pub type_field: Type,
        }
        let LogLine { type_field } = serde_json::from_str(&value).map_err(Error::from)?;

        Ok(Message {
            type_field,
            byte_size: value.as_bytes().len() as u64,
        })
    }
}
