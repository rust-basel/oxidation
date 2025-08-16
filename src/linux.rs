use std::path::PathBuf;

pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("oxidation"))
}
