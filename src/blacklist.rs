use std::{error::Error, path::Path, fs, io};

use toml::Table;

const DEFAULT_CONFIG: &str = r#"# entries should be comma separated and put in double quotes

title_blacklist = [
    # examples for valid entries (Steam subwindows)
    #"Friends List",
    #"Shutdown"
]"#;

pub fn init_blacklist_file() -> Result<(), io::Error> {
    let cfg = get_blacklist_path()?;
    if !Path::new(&cfg).exists() {
        fs::write(&cfg, DEFAULT_CONFIG)?;
    }

    Ok(())
}

fn get_blacklist_path() -> Result<String, io::Error> {
    Ok(
        format!("{}/.config/xwl-notifier.toml", dirs::home_dir()
            .ok_or(io::ErrorKind::NotFound)?
            .to_str()
            .ok_or(io::ErrorKind::InvalidData)?
            .to_owned())
    )
}

pub fn read_blacklist() -> Result<Vec<String>, Box<dyn Error>> {
    let cfg = get_blacklist_path()?;
    let contents = fs::read_to_string(cfg)?;

    let table: Table = toml::from_str(&contents)?;

    let blacklist: Vec<String> = table
        .into_iter()
        .next()
        .and_then(|(_key, value)| value.try_into().ok())
        .unwrap_or_default();

    return Ok(blacklist);
}