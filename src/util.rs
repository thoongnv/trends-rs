use dirs;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;

pub fn get_api_key() -> Result<String, std::io::Error> {
    let home_dir = dirs::home_dir().unwrap();
    let home_dir_str = home_dir.to_str().unwrap();
    let config_dir;

    if Path::new(&format!("{}/.shodan", home_dir_str)).is_dir() {
        config_dir = format!("{}/.shodan", home_dir_str);
    } else {
        config_dir = format!("{}/.config/shodan", home_dir_str);
    }

    let mut file = File::open(format!("{}/api_key", config_dir))?;
    let mut api_key = String::new();
    file.read_to_string(&mut api_key)?;

    Ok(api_key
        .strip_suffix("\r\n")
        .or(api_key.strip_suffix("\n"))
        .unwrap_or(&api_key)
        .to_owned())
}
