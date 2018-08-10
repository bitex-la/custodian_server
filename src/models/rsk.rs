use std::process;
use serde_json;

pub struct Rsk;

impl Rsk {
    pub fn get_temp_addresses(&self) -> serde_json::Value {
        let output = process::Command::new("rsk-tool")
            .arg("-n")
            .output()
            .expect("Command not found");

        let raw_value = String::from_utf8_lossy(&output.stdout).to_string();
        serde_json::from_str(&str::replace(&raw_value, "'", "\"")).expect("Problem deserializing object")
    }
}
