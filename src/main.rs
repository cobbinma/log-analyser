use anyhow::{Context, Error};
use comfy_table::Table;
use models::TypeStatistic;
use options::Opt;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Write},
    path::PathBuf,
};
use structopt::StructOpt;

mod analyse;
mod models;
mod options;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let options = Opt::from_args();
    let file = File::open(options.input_file).context("unable to open input file")?;

    let statistics = analyse::lines(io::BufReader::new(file).lines()).await;

    output_errors(options.error_file, statistics.errors()).context("unable to output errors")?;

    output_type_statistics(options.output_file, statistics.types())
        .context("unable to output statistics")?;

    Ok(())
}

fn output_errors(output_file: Option<PathBuf>, errors: &[anyhow::Error]) -> Result<(), Error> {
    let mut table = Table::new();
    table.set_header(vec!["Errors"]);

    errors.iter().for_each(|e| {
        table.add_row(vec![format!("{:#}", e)]);
    });

    match output_file {
        Some(file) => {
            let mut file = File::create(file).context("unable to create file")?;
            file.write_all(table.to_string().as_bytes())
                .context("unable to write file")?;
        }
        None => eprintln!("{}", table),
    };

    Ok(())
}

fn output_type_statistics(
    output_file: Option<PathBuf>,
    stats: &HashMap<String, TypeStatistic>,
) -> Result<(), Error> {
    let mut table = Table::new();
    table.set_header(vec!["Type", "Total Messages", "Total Byte Size"]);

    stats.iter().for_each(|(type_field, statistic)| {
        table.add_row(vec![
            type_field,
            &statistic.total_messages.to_string(),
            &statistic.total_byte_size.to_string(),
        ]);
    });

    match output_file {
        Some(file) => {
            let mut file = File::create(file).context("unable to create file")?;
            file.write_all(table.to_string().as_bytes())
                .context("unable to write file")?;
        }
        None => println!("{}", table),
    };

    Ok(())
}
