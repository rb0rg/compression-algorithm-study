use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::constants;

pub fn command() -> Command {
    Command::new("decompress")
        .about(format!(
            "Decompress a .{} into it's original file",
            constants::file_constants::FILE_EXTENSION_TO_LOOKUP
        ))
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help(format!(
                    ".{} file to decompress",
                    constants::file_constants::FILE_EXTENSION_TO_LOOKUP
                )),
        )
}

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let input: &PathBuf = matches.get_one("input").expect("Input file is required");

    if !input.exists() {
        anyhow::bail!("Input file does not exist: {}", input.display());
    }

    if input.extension().expect("Input file has no extension")
        != constants::file_constants::FILE_EXTENSION_TO_LOOKUP
    {
        anyhow::bail!(
            "Input file is not a .{} file: {}",
            constants::file_constants::FILE_EXTENSION_TO_LOOKUP,
            input.display()
        );
    }

    println!("Decompress file: {}", input.display());

    Ok(())
}
