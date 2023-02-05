use std::collections::HashMap;

use serde_json::Value;

use crate::chooser::ChooserPlugin;
use crate::config::FrequencyBandMap;

pub const NAME: &'static str = "single";

pub struct SingleChooserPlugin {}

impl SingleChooserPlugin {
    pub fn new() -> &'static Self {
        &SingleChooserPlugin {}
    }
}

impl ChooserPlugin for SingleChooserPlugin {
    fn choose<'a, 'b>(
        &self,
        bands: &'a FrequencyBandMap,
        props: &'b HashMap<&str, &str>,
    ) -> Result<&'a Vec<u32>, String> {
        if !props.contains_key("band") {
            return Err("Missing 'band' key in props".to_string());
        }

        let band: u32 = match props.get("band").unwrap().parse() {
            Ok(band) => band,
            Err(e) => return Err(format!("'band' is not a valid positive number: {}", e)),
        };

        bands.get(&band).ok_or(format!("Invalid band: {}", band))
    }

    fn on_update(&self, _frame: &Value) -> bool {
        false
    }

    fn on_timeout(&self) -> bool {
        false
    }
}
