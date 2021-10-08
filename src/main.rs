use crate::models::{Message, TypeStat};
use anyhow::{Context, Error};
use futures::{
    stream::{self, StreamExt},
};
use std::{collections::HashMap, convert::TryFrom, fs::File, io::{self, BufRead}, ops::Add};

mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file = File::open("input.log")?;

    let stats: HashMap<String, TypeStat> = stream::iter(io::BufReader::new(file).lines())
        .then(|line| async {
            line.context("unable to read line")
                .map_err(From::from)
                .and_then(Message::try_from)
                .context("unable to parse line")
        })
        .fold(HashMap::new(), |mut acc, message| async move {
            let message: Message = message.unwrap_or(Message {
                type_field: "error".to_string(),
                byte_size: 0,
            });
            let type_field = message.type_field.clone();
            acc.entry(type_field.clone()).or_insert(TypeStat{ type_field, total_messages: 0, total_byte_size: 0 }).add_message(&message);
            
            acc
        })
        .await;

    for (_, stat) in stats {
        println!("{:?}", stat);
    }

    Ok(())
}
