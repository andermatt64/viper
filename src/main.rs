use crossbeam::channel::{after, bounded, select};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime};
use tempfile::NamedTempFile;

use clap::Parser;
use log::*;

mod args;
mod chooser;
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
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to parser configuration: {}", e);
            return;
        }
    };

    let (name, props) = args.chooser_params();

    let chooser = match chooser::get(name) {
        Some(plugin) => plugin,
        None => {
            error!("Invalid plugin name: {}", name);
            return;
        }
    };

    let mut systable = match NamedTempFile::new() {
        Ok(fd) => fd,
        Err(e) => {
            error!("Unable to create temporary file for systable config: {}", e);
            return;
        }
    };
    write!(systable, "{}", config.info.raw).unwrap();
    systable.seek(SeekFrom::Start(0)).unwrap();

    let systable_temp_path = systable.into_temp_path();

    loop {
        let band = match chooser.choose(&config.info.bands, &props) {
            Ok(val) => val.clone(),
            Err(e) => {
                error!("Failed to choose a frequency band to listen to: {}", e);
                return;
            }
        };
        let mut proc = match Command::new(config.bin.clone())
            .stdout(Stdio::piped())
            .arg("--soapysdr")
            .arg(config.driver.clone())
            .arg("--system-table")
            .arg(systable_temp_path.to_path_buf())
            .arg("--output")
            .arg("decoded:json:file:path=-")
            .arg("--sample-rate")
            .arg("384000")
            .args(band.into_iter().map(|f| f.to_string()))
            .spawn()
        {
            Ok(proc) => proc,
            Err(e) => {
                error!("Failed to start dumphfdl: {}", e);
                continue;
            }
        };

        let (frame_send, frame_recv) = bounded(2048);
        let child_stdout = match proc.stdout.take() {
            Some(stdout) => stdout,
            None => {
                error!("Unable to get STDOUT for child dumphfdl process!");
                continue;
            }
        };
        let reader_thread = thread::spawn(move || {
            let mut reader = BufReader::new(child_stdout);

            loop {
                let mut line = String::new();
                let size = match reader.read_line(&mut line) {
                    Ok(size) => size,
                    Err(e) => {
                        error!("Reader thread encountered read error: {}", e);
                        break;
                    }
                };
                if size == 0 {
                    break;
                }

                if frame_send.send(line).is_err() {
                    break;
                }
            }
        });

        let timeout = Duration::from_secs((config.timeout as u64) * 60);

        loop {
            select! {
                recv(frame_recv) -> msg => {
                    println!("{}", msg.unwrap());
                },
                recv(after(timeout)) -> _ => {
                    if chooser.on_timeout() {
                        break;
                    }
                },
            }
        }

        proc.kill().unwrap();
        reader_thread.join().unwrap();

        break;
    }
}
