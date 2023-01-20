use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to dumphfdl binary
    #[arg(short, long, value_name = "FILE", default_value = "/usr/bin/dumphfdl")]
    bin: Option<PathBuf>,

    /// Path to dumphfdl system table configuration
    #[arg(short, long, value_name = "FILE", default_value = "/etc/systable.conf")]
    table: Option<PathBuf>,

    /// Verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Silence all output
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

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

    println!("args = {:?}", args)
}
