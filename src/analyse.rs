use anyhow::Context;
use std::convert::TryFrom;
use tracing::error;

use futures::{
    stream::{self},
    StreamExt,
};

use crate::models::{Message, Statistics};

pub async fn lines<I>(lines: I) -> Statistics
where
    I: IntoIterator<Item = Result<String, std::io::Error>>,
{
    stream::iter(lines)
        .then(|line| async {
            let message = line
                .context("unable to read line")
                .map_err(From::from)
                .and_then(Message::try_from)
                .context("unable to parse line");

            if let Err(error) = &message {
                let error_message = format!("{:#}", error);
                error!(message = "error analysing line", %error_message);
            };

            message
        })
        .fold(Statistics::new(), |mut stats, message| async move {
            match message {
                Ok(m) => stats.add_message(m),
                Err(e) => stats.add_error(e),
            }

            stats
        })
        .await
}

#[cfg(test)]
mod tests {
    use crate::models::TypeStatistic;

    use super::lines;

    #[tokio::test]
    async fn test_lines() {
        let statistics = lines(vec![
            Ok(r#"{"type":"B","foo":"bar","items":["one","two"]}"#.to_string()),
            Ok(r#"{"type": "A","foo": 4.0 }"#.to_string()),
            Ok(r#"{"type": "B","bar": "abcd"}"#.to_string()),
            Ok(r#"{"type": "B","bar": "abcd"#.to_string()),
        ])
        .await;

        let types = statistics.types();

        assert_eq!(
            types.get("A"),
            Some(&TypeStatistic {
                total_messages: 1,
                total_byte_size: 25
            })
        );
        assert_eq!(
            types.get("B"),
            Some(&TypeStatistic {
                total_messages: 2,
                total_byte_size: 73
            })
        );
        assert_eq!(types.get("C"), None);
        assert_eq!(statistics.errors().len(), 1);
    }
}
