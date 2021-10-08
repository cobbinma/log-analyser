use anyhow::Context;
use std::convert::TryFrom;

use futures::{
    stream::{self},
    StreamExt,
};

use crate::models::{Message, Statistics, TypeStatistic};

pub async fn messages<I>(messages: I) -> Statistics
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
        .fold(Statistics::new(), |mut stats, message| async move {
            match message {
                Ok(message) => stats
                    .types
                    .entry(message.type_field.clone())
                    .or_insert(TypeStatistic {
                        total_messages: 0,
                        total_byte_size: 0,
                    })
                    .add_message(&message),
                Err(e) => stats.errors.push(e),
            }

            stats
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::messages;

    #[tokio::test]
    async fn my_test() {
        let statistics = messages(vec![
            Ok(r#"{"type":"B","foo":"bar","items":["one","two"]}"#.to_string()),
            Ok(r#"{"type": "A","foo": 4.0 }"#.to_string()),
            Ok(r#"{"type": "B","bar": "abcd"}"#.to_string()),
        ])
        .await;

        assert_eq!(statistics.types.len(), 2);
        assert_eq!(statistics.errors.len(), 0);
    }
}
