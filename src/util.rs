use dirs;
use serde_json::{json, Value};
use std::fs::{create_dir_all, metadata, set_permissions, File};
use std::io::prelude::*;
use std::path::Path;

pub fn get_config_dir() -> String {
    let home_dir = dirs::home_dir().unwrap().display().to_string();
    match Path::new(&format!("{}/.shodan", home_dir)).is_dir() {
        true => format!("{}/.shodan", home_dir),
        false => format!("{}/.config/shodan", home_dir),
    }
}

pub fn get_api_key() -> Result<String, std::io::Error> {
    let config_dir: String = get_config_dir();
    let mut file = File::open(format!("{}/api_key", config_dir))?;
    let mut api_key = String::new();
    file.read_to_string(&mut api_key)?;

    Ok(api_key
        .strip_suffix("\r\n")
        .or(api_key.strip_suffix('\n'))
        .unwrap_or(&api_key)
        .to_owned())
}

pub fn init_api_key(mut key: String, validate: bool) -> Result<(), std::io::Error> {
    // Check if API key is valid
    key = key.trim().to_owned();
    let mut valid = false;

    if validate {
        let resp: Result<ureq::Response, ureq::Error> = ureq::get("https://api.shodan.io/api-info")
            .query("key", &key)
            .call();

        match resp {
            Ok(_) => {
                valid = true;
            }
            Err(ureq::Error::Status(_, response)) => {
                let resp_str = response.into_string()?;
                let error: Value = serde_json::from_str(&resp_str).unwrap_or(json!({
                    "error": "Invalid API key",
                }));
                println!("Error: {}", error["error"].as_str().unwrap());
            }
            Err(_) => {
                println!("Error: Failed to validate API key");
            }
        }
    } else {
        // Force save key
        valid = true;
    }

    if valid {
        // Create the directory if missing
        let config_dir: String = get_config_dir();
        create_dir_all(config_dir.clone())?;

        // Save key to file
        let fpath = format!("{}/api_key", config_dir);
        let mut file = File::create(fpath.clone())?;
        file.write_all(key.as_bytes())?;

        // Set permission skip if errored out
        if let Ok(metadata) = metadata(fpath.clone()) {
            let mut perms = metadata.permissions();
            perms.set_readonly(true);

            let _ = set_permissions(fpath, perms);
        };

        println!("Successfully initialized");
    }

    Ok(())
}
