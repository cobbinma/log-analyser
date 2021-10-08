use crate::models::Log;
use anyhow::{Context, Error};
use futures::stream::{self, StreamExt};
use std::{
    convert::TryFrom,
    fs::File,
    io::{self, BufRead},
};

mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let file = File::open("input.log")?;

    let logs = stream::iter(io::BufReader::new(file).lines())
        .then(|line| async {
            line
                .context("unable to read line")
                .map_err(From::from)
                .and_then(Log::try_from)
                .context("unable to parse line")
        })
        .collect::<Vec<_>>()
        .await;

    println!("{:?}", logs);

    Ok(())
}
