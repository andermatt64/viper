use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};
use url::Url;

pub type GroundStationMap = HashMap<String, GroundStation>;
pub type FrequencyBandMap = HashMap<u32, Vec<u32>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct GroundStation {
    id: u32,
    name: String,
    lat: f64,
    lon: f64,
}

impl GroundStation {
    pub fn as_wkt(&self) -> String {
        format!("POINT ({} {})", self.lon, self.lat)
    }

    pub fn as_point(&self) -> String {
        format!("{} {}", self.lon, self.lat)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HFDLInfo {
    pub stations: GroundStationMap,
    pub bands: FrequencyBandMap,
    pub raw: String,
}

#[derive(Debug)]
pub struct Config {
    pub bin: PathBuf,
    pub driver: String,
    pub timeout: u32,

    pub es_idx: String,
    pub es_url: Url,

    pub info: HFDLInfo,
}

impl Config {
    fn parse_systable(path: &PathBuf) -> Result<HFDLInfo, String> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(e) => return Err(format!("Unable to read dumphfdl system table: {}", e)),
        };

        serde_json::from_str(&contents)
            .map_err(|e| format!("Unable to deserialize dumphfdl system table: {}", e))
    }

    pub fn from_args(args: &crate::args::Args) -> Result<Config, String> {
        if !args.bin.exists() || !args.bin.is_file() {
            return Err(format!(
                "dumphfdl binary path does not exist or is not a file: {:?}",
                args.bin
            ));
        }
        if !args.sys_table.exists() || !args.bin.is_file() {
            return Err(format!(
                "dumphfdl system table definition does not exist or is not a file: {:?}",
                args.sys_table
            ));
        }

        let soapy_driver = env::var("VIPER_SOAPY_DRIVER").map_or_else(
            |_| args.driver.clone(),
            |val| {
                if val.len() > 0 {
                    val
                } else {
                    args.driver.clone()
                }
            },
        );
        let es_idx = env::var("VIPER_ES_IDX").map_or_else(
            |_| args.es_idx.clone(),
            |val| {
                if val.len() > 0 {
                    val
                } else {
                    args.es_idx.clone()
                }
            },
        );
        let es_url_str = env::var("VIPER_ES_URL").map_or_else(
            |_| args.es_url.clone(),
            |val| {
                if val.len() > 0 {
                    val
                } else {
                    args.es_url.clone()
                }
            },
        );
        let es_url = match url::Url::parse(es_url_str.as_str()) {
            Ok(url) => url,
            Err(e) => return Err(format!("ElasticSearch URL is not valid: {}", e)),
        };
        if es_url.cannot_be_a_base() {
            return Err(format!(
                "ElasticSearch URL cannot be a data URL: {}",
                Into::<String>::into(es_url)
            ));
        }
        if !["http", "https"].contains(&es_url.scheme()) {
            return Err(format!(
                "ElasticSearch URL must start with http or https: {}",
                Into::<String>::into(es_url)
            ));
        }

        let info = Config::parse_systable(&args.sys_table)?;

        Ok(Config {
            bin: args.bin.clone(),
            driver: soapy_driver,
            timeout: args.timeout,
            es_url,
            es_idx,
            info,
        })
    }
}
