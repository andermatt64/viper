use crate::config::FrequencyBandMap;
use serde_json::Value;
use std::collections::HashMap;

pub mod single;

pub trait ChooserPlugin {
    fn choose<'a, 'b>(
        &self,
        bands: &'a FrequencyBandMap,
        props: &'b HashMap<&str, &str>,
    ) -> Result<&'a Vec<u32>, String>;

    fn on_update(&self, frame: &Value) -> bool;
    fn on_timeout(&self) -> bool;
}

pub fn get(name: &str) -> Option<&'static dyn ChooserPlugin> {
    match name {
        single::NAME => Some(single::SingleChooserPlugin::new()),
        _ => None,
    }
}
