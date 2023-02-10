use log::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use serde::Deserialize;
use serde_json::Value;

use crate::chooser::ChooserPlugin;
use crate::config::FrequencyBandMap;

pub const NAME: &'static str = "target";
pub const MAX_VISITED_ENTRIES: usize = 6;

#[derive(Deserialize, Debug)]
struct Frequency {
    id: u8,
    freq: f64,
}

#[derive(Deserialize, Debug)]
struct Entity {
    id: u8,

    #[serde(alias = "type")]
    entity_type: String,

    #[serde(alias = "name")]
    entity_name: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GroundStation {
    gs: Entity,
    utc_sync: bool,
    freqs: Vec<Frequency>,
}

#[derive(Deserialize, Debug)]
struct LPDU {
    err: bool,
    src: Entity,
    dst: Entity,
}

#[derive(Deserialize, Debug)]
struct SPDU {
    err: bool,
    src: Entity,
    gs_status: Vec<GroundStation>,
}

#[derive(Deserialize, Debug)]
struct HFDL {
    spdu: Option<SPDU>,
    lpdu: Option<LPDU>,
}

#[derive(Deserialize, Debug)]
struct MessageFrame {
    hfdl: HFDL,
}

pub struct TargetChooserPlugin {
    recently_visited: Vec<u32>,
    gs_last_heard: Option<Instant>,
    target: Option<String>,
    next_band: Option<u32>,
}

impl TargetChooserPlugin {
    pub fn new() -> Self {
        TargetChooserPlugin {
            recently_visited: vec![],
            gs_last_heard: None,
            target: None,
            next_band: None,
        }
    }
}

impl ChooserPlugin for TargetChooserPlugin {
    fn choose<'a, 'b>(
        &mut self,
        bands: &'a FrequencyBandMap,
        props: &'b HashMap<&str, &str>,
    ) -> Result<&'a Vec<u32>, String> {
        if self.target.is_none() {
            self.target = props.get("target").map(|s| s.to_string());
            if self.target.is_none() {
                return Err("No target specified".to_string());
            }
        }

        let next_band: u32;

        if self.next_band.is_none() {
            let mut rng = rand::thread_rng();
            let mut band_keys: Vec<&u32> = bands.keys().into_iter().collect();
            band_keys.shuffle(&mut rng);

            while !self.recently_visited.is_empty()
                && self
                    .recently_visited
                    .iter()
                    .position(|&b| b == *band_keys[0])
                    .is_some()
            {
                band_keys.remove(0);
            }

            next_band = *band_keys[0];
        } else {
            next_band = self.next_band.unwrap();
        }

        if self.recently_visited.len() == MAX_VISITED_ENTRIES {
            self.recently_visited.remove(0);
        }
        self.recently_visited.push(next_band);

        bands
            .get(&next_band)
            .ok_or(format!("Invalid band: {}", next_band))
    }

    fn on_update(&self, frame: &Value) -> bool {
        let msg: MessageFrame = match serde_json::from_value(frame.clone()) {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to coerce frame into MessageFrame: {}", e);
                return false;
            }
        };

        println!("{:?}", msg);

        false
    }

    fn on_timeout(&self) -> bool {
        true
    }
}
