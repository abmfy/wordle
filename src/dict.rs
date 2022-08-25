use lazy_static::lazy_static;
use serde_json;
use std::collections::HashMap;

lazy_static! {
    pub static ref DICT: HashMap<String, Vec<String>> = {
        let json = include_str!("../assets/dict.json");
        serde_json::from_str(json).unwrap()
    };
}
