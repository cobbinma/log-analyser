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
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

mod analyse;
mod models;
mod options;

fn main() -> Result<(), Error> {
    let options = Opt::from_args();

    if options.debug {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("debug"))
            .init();
    };

    info!(message = "starting log-analyser", ?options);

    let file = File::open(options.input_file).context("unable to open input file")?;

    let statistics = analyse::lines(io::BufReader::new(file).lines());

    output_errors(options.error_file, statistics.errors()).context("unable to output errors")?;

    output_type_statistics(options.output_file, statistics.types())
        .context("unable to output statistics")?;

    match statistics.errors().is_empty() {
        true => Ok(()),
        false => Err(anyhow::anyhow!(format!(
            "input file included {} error(s)",
            statistics.errors().len()
        ))),
    }
}

fn output_errors(errors_file: Option<PathBuf>, errors: &[anyhow::Error]) -> Result<(), Error> {
    debug!("outputting errors : found {} error(s)", errors.len());
    if !errors.is_empty() {
        let mut table = Table::new();
        table.set_header(vec!["Errors"]);

        errors.iter().for_each(|e| {
            table.add_row(vec![format!("{:#}", e)]);
        });

        match errors_file {
            Some(file) => {
                debug!(message = "outputting errors to file", ?file);
                let mut file = File::create(file).context("unable to create file")?;
                file.write_all(table.to_string().as_bytes())
                    .context("unable to write file")?;
            }
            None => {
                debug!("outputting errors to stderr");
                eprintln!("{}", table);
            }
        };
    };

    Ok(())
}

fn output_type_statistics(
    output_file: Option<PathBuf>,
    stats: &HashMap<String, TypeStatistic>,
) -> Result<(), Error> {
    debug!("outputting statistics : found {} type(s)", stats.len());
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
            debug!(message = "outputting statistics to file", ?file);
            let mut file = File::create(file).context("unable to create file")?;
            file.write_all(table.to_string().as_bytes())
                .context("unable to write file")?;
        }
        None => {
            debug!("outputting statistics to stdout");
            println!("{}", table);
        }
    };

    Ok(())
}
