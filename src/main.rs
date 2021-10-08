use anyhow::{Context, Error};
use comfy_table::Table;
use models::TypeStatistic;
use options::Opt;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};
use structopt::StructOpt;

mod analyse;
mod models;
mod options;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let options = Opt::from_args();
    let file = File::open(options.input).context("unable to open input file")?;

    print_statistics(analyse::messages(io::BufReader::new(file).lines()).await);

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
