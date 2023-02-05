use std::collections::HashMap;

use serde_json::Value;

use crate::chooser::ChooserPlugin;
use crate::config::FrequencyBandMap;

pub const NAME: &'static str = "rotate";

pub struct RotateChooserPlugin {
    band_idx: Option<usize>,
}

impl RotateChooserPlugin {
    pub fn new() -> Self {
        RotateChooserPlugin { band_idx: None }
    }
}

impl ChooserPlugin for RotateChooserPlugin {
    fn choose<'a, 'b>(
        &mut self,
        bands: &'a FrequencyBandMap,
        props: &'b HashMap<&str, &str>,
    ) -> Result<&'a Vec<u32>, String> {
        let mut band_keys: Vec<&u32> = bands.keys().into_iter().collect();
        band_keys.sort_unstable();

        if self.band_idx.is_none() {
            let start: u32 = match props.get("start").unwrap_or(&"13").parse() {
                Ok(start) => start,
                Err(e) => {
                    return Err(format!(
                        "'start' key contains an invalid positive number: {}",
                        e
                    ))
                }
            };

            self.band_idx = band_keys.iter().position(|&b| b == &start);
            if self.band_idx.is_none() {
                return Err(format!("'start' key value ({}) is not a valid band", start));
            }
        } else {
            if self.band_idx.unwrap() + 1 >= band_keys.len() {
                self.band_idx = Some(0);
            } else {
                self.band_idx = Some(self.band_idx.unwrap() + 1);
            }
        }

        let band = band_keys[self.band_idx.unwrap()];
        bands.get(&band).ok_or(format!("Invalid band: {}", band))
    }

    fn on_update(&self, _frame: &Value) -> bool {
        false
    }

    fn on_timeout(&self) -> bool {
        true
    }
}
