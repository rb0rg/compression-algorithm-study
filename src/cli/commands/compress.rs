use std::{collections::HashMap, fs, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

use crate::{constants, core};

pub fn command() -> Command {
    Command::new("compress")
        .about(format!(
            "Compresses a file into .{}",
            constants::file_constants::FILE_EXTENSION_TO_LOOKUP
        ))
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help("The file to compress"),
        )
}

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let input: &PathBuf = matches.get_one("input").expect("Input file is required");

    if !input.exists() {
        anyhow::bail!("Input file does not exist: {}", input.display());
    }

    println!("Compress file: {}", input.display());

    let file_bytes: Vec<u8> = fs::read(input)?;

    let _freq_map: HashMap<u8, usize> =
        core::algorithm::huffman::generate_frequency_map(&file_bytes);

    Ok(())
}
