use crate::models::Log;
use anyhow::Error;
use std::{
    convert::TryFrom,
    fs::File,
    io::{self, BufRead},
};

mod models;

fn main() -> Result<(), Error> {
    let file = File::open("input.log")?;
    for line in io::BufReader::new(file).lines() {
        println!("{:?}", line.map_err(From::from).and_then(Log::try_from));
    }

    Ok(())
}
