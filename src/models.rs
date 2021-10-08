use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Log {
    pub type_field: String,
    pub byte_size: usize,
}

impl TryFrom<String> for Log {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Error> {
        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct LogLine {
            #[serde(rename = "type")]
            pub type_field: String,
        }
        let LogLine { type_field } = serde_json::from_str(&value).map_err(Error::from)?;

        Ok(Log {
            type_field,
            byte_size: value.as_bytes().len(),
        })
    }
}
