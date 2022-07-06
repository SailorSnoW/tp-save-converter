use std::error::Error;
use std::fs::{read, write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use clap::Parser;
use crate::error::Error::NotGciFile;

mod args;
mod error;

use args::Args;

const RANGE_QUESTLOG1: Range<usize> = 0x4048..0x4AD3;
const RANGE_QUESTLOG2: Range<usize> = 0x4ADC..0x5567;
const RANGE_QUESTLOG3: Range<usize> = 0x5570..0x5FFB;

fn main() -> Result<(), Box<dyn Error>> {
    let arg_path = Args::parse().path;
    let gci_path: &Path = Path::new(&arg_path);

    match gci_path.extension() {
        Some(ext) => {
            match ext.to_str() {
                Some("gci") => (),
                _ => return Err(Box::new(NotGciFile))
            }
        },
        None => return Err(Box::new(NotGciFile))
    }

    let buffer = read(gci_path)?;

    let save1 = &buffer[RANGE_QUESTLOG1];
    let save2 = &buffer[RANGE_QUESTLOG2];
    let save3 = &buffer[RANGE_QUESTLOG3];

    let filename1 = parse_to_path(&Args::parse().save1);
    let filename2 = parse_to_path(&Args::parse().save2);
    let filename3 = parse_to_path(&Args::parse().save3);

    println!("creating {:?}...", filename1);
    write(filename1, save1)?;
    println!("creating {:?}...", filename2);
    write(filename2, save2)?;
    println!("creating {:?}...", filename3);
    write(filename3, save3)?;

    println!("success");
    Ok(())
}

fn parse_to_path(s: &str) -> PathBuf {
    PathBuf::from(
        format!("{}{}", s, ".bin")
    )
}
