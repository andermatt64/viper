use clap::Parser;
use log::*;

mod args;
mod config;

fn main() {
    let args = args::Args::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(if args.verbose { 3 } else { 1 })
        .timestamp(if args.verbose {
            stderrlog::Timestamp::Second
        } else {
            stderrlog::Timestamp::Off
        })
        .init()
        .unwrap();

    let config = match config::Config::from_args(&args) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to parse configuration: {}", e);
            return;
        }
    };
    println!("{:?}", config);
}
