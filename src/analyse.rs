use anyhow::Context;
use std::convert::TryFrom;
use tracing::error;

use crate::models::{Message, MessageStatistics};

pub fn lines<I>(lines: I) -> MessageStatistics
where
    I: Iterator<Item = Result<String, std::io::Error>>,
{
    lines
        .map(|line| {
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
        .fold(MessageStatistics::new(), |mut stats, message| {
            match message {
                Ok(m) => stats.add_message(m),
                Err(e) => stats.add_error(e),
            }

            stats
        })
}

#[cfg(test)]
mod tests {
    use crate::models::TypeStatistic;

    use super::lines;

    #[test]
    fn test_lines() {
        let statistics = lines(
            vec![
                Ok(r#"{"type":"B","foo":"bar","items":["one","two"]}"#.to_string()),
                Ok(r#"{"type": "A","foo": 4.0 }"#.to_string()),
                Ok(r#"{"type": "B","bar": "abcd"}"#.to_string()),
                Ok(r#"{"type": "B","bar": "abcd"#.to_string()),
            ]
            .into_iter(),
        );

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
