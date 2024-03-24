use std::process;

use blockchain_simulator::Config;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Path to config file
    #[arg(short, long)]
    config: String,
}

fn main() {
    let args = Args::parse();

    let config = Config::from_file(&args.config).unwrap_or_else(|err| {
        eprintln!("error during config parsing: {err}");
        process::exit(1);
    });

    blockchain_simulator::run(config).unwrap_or_else(|err| {
        eprintln!("error during execution: {err}");
        process::exit(1);
    });
}
