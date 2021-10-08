use anyhow::{Context, Error};
use comfy_table::Table;
use futures::stream::{self, StreamExt};
use models::{Message, TypeStatistic};
use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::File,
    io::{self, BufRead},
};

mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file = File::open("input.log")?;

    let stats: HashMap<String, TypeStatistic> = stream::iter(io::BufReader::new(file).lines())
        .then(|line| async {
            line.context("unable to read line")
                .map_err(From::from)
                .and_then(Message::try_from)
                .context("unable to parse line")
        })
        .fold(HashMap::new(), |mut stats, message| async move {
            let message: Message = message.unwrap_or(Message::error());
            stats
                .entry(message.type_field.clone())
                .or_insert(TypeStatistic {
                    total_messages: 0,
                    total_byte_size: 0,
                })
                .add_message(&message);

            stats
        })
        .await;

    print_statistics(stats);

    Ok(())
}

fn print_statistics(stats: HashMap<String, TypeStatistic>) {
    let mut table = Table::new();
    table.set_header(vec!["Type", "Total Messages", "Total Byte Size"]);

    stats.iter().for_each(|(type_field, statistic)| {
        table.add_row(vec![
            type_field,
            &statistic.total_messages.to_string(),
            &statistic.total_byte_size.to_string(),
        ]);
    });

    println!("{}", table);
}
