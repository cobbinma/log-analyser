use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "log-analyser")]
pub struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    pub input_file: PathBuf,

    /// Output file
    #[structopt(short, long)]
    pub output_file: Option<PathBuf>,

    /// Error file
    #[structopt(short, long)]
    pub error_file: Option<PathBuf>,
}
