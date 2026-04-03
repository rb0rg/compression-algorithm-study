use clap::{ArgMatches, Command};

mod commands;

pub fn build_cli() -> Command {
    Command::new("compression-algorithm")
        .about("Rafael Borges's study on compression algorithms as a CLI tool.")
        .subcommand_required(true)
        .subcommand(commands::compress::command())
        .subcommand(commands::decompress::command())
}

pub fn handle_matches(matches: ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand() {
        Some(("compress", sub_m)) => commands::compress::run(sub_m)?,
        Some(("decompress", sub_m)) => commands::decompress::run(sub_m)?,
        _ => println!("No subcommand was used!"),
    }

    Ok(())
}
