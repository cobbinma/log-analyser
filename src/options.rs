use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "log-analyser")]
pub struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,
}
