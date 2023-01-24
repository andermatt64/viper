use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to dumphfdl binary
    #[arg(long, value_name = "FILE", default_value = "/usr/bin/dumphfdl")]
    pub bin: PathBuf,

    /// Path to dumphfdl system table configuration
    #[arg(long, value_name = "FILE", default_value = "/etc/systable.conf")]
    pub sys_table: PathBuf,

    /// ElasticSearch index name (override w/ VIPER_ES_IDX)
    #[arg(long, value_name = "INDEX_NAME", default_value = "hfdl_db")]
    pub es_idx: String,

    /// ElasticSearch URL (override w/ VIPER_ES_URL)
    #[arg(long, value_name = "URL", default_value = "http://localhost:5900")]
    pub es_url: String,

    /// SoapySDR driver configuration (override w/ VIPER_SOAPY_DRIVER)
    #[arg(long, value_name = "DRIVER", default_value = "driver=airspyhf")]
    pub driver: String,

    /// Methodology for changing HFDL bands (override w/ VIPER_CHOOSER)
    #[arg(
        long,
        value_name = "PLUGIN_NAME[:KEY=VALUE,...]",
        default_value = "default"
    )]
    pub chooser: String,

    /// Verbose mode
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Silence all output
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,

    /// Timeout in minutes to wait before switching HF bands
    #[arg(short, long, value_name = "MINUTES", default_value_t = 5)]
    pub timeout: u32,
}
