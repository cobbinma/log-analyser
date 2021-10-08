use crate::models::Log;
use anyhow::{Context, Error};
use std::{
    convert::TryFrom,
    fs::File,
    io::{self, BufRead},
};

mod models;

fn main() -> Result<(), Error> {
    let file = File::open("input.log")?;
    for line in io::BufReader::new(file).lines() {
        println!(
            "{:?}",
            line.context("unable to read line")
                .map_err(From::from)
                .and_then(Log::try_from)
                .context("unable to parse line")
        );
    }

    Ok(())
}
