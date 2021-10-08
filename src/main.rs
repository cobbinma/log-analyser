use std::{fs::File, io::{self, BufRead}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.log")?;
    for line in io::BufReader::new(file).lines() {
        println!("{}", line?)
    }

    Ok(())
}
