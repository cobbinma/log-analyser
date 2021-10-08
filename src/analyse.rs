use anyhow::Context;
use std::{collections::HashMap, convert::TryFrom};

use futures::{
    stream::{self},
    StreamExt,
};

use crate::models::{Message, TypeStatistic};

pub async fn messages<I>(messages: I) -> HashMap<String, TypeStatistic>
where
    I: IntoIterator<Item = Result<String, std::io::Error>>,
{
    stream::iter(messages)
        .then(|line| async {
            line.context("unable to read line")
                .map_err(From::from)
                .and_then(Message::try_from)
                .context("unable to parse line")
        })
        .fold(HashMap::new(), |mut stats, message| async move {
            let message: Message = message.unwrap_or_else(|_| Message::error());
            stats
                .entry(message.type_field.clone())
                .or_insert(TypeStatistic {
                    total_messages: 0,
                    total_byte_size: 0,
                })
                .add_message(&message);

            stats
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::messages;

    #[tokio::test]
    async fn my_test() {
        messages(vec![
            Ok(r#"{"type":"B","foo":"bar","items":["one","two"]}"#.to_string()),
            Ok(r#"{"type": "A","foo": 4.0 }"#.to_string()),
            Ok(r#"{"type": "B","bar": "abcd"}"#.to_string()),
            ]).await;
    }
}