use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::{collections::HashMap, fs, io::Write, path::PathBuf, process};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    src: String,
    import_aliases: bool,
    use_cache: bool,
    aliases: HashMap<String, String>,
    paths: HashMap<String, String>,
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            src: "src".to_string(),
            import_aliases: true,
            use_cache: true,
            aliases: HashMap::new(),
            paths: HashMap::new(),
        }
    }
}

impl UserConfig {
    pub fn save_to(&self, path: &PathBuf) {
        let json = to_string_pretty(self).unwrap_or_else(|e| {
            eprintln!("failed to serialize config {}", e);
            process::exit(1);
        });
        let mut file = fs::File::create(path).unwrap_or_else(|e| {
            eprintln!("failed to make file {}", e);
            process::exit(1);
        });
        file.write_all(json.as_bytes()).unwrap_or_else(|e| {
            eprintln!("failed to write to file {}", e);
            process::exit(1);
        });
    }

    pub fn load_from(path: &PathBuf) -> UserConfig {
        if !path.exists() {
            println!("load path doesn't exist. using default config");
            return UserConfig::default();
        }
        let data = fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("failed to read from file {}", e);
            process::exit(1);
        });
        let config = from_str::<UserConfig>(&data).unwrap_or_else(|e| {
            eprintln!("failed to deserialize json data {}", e);
            process::exit(1);
        });
        return config;
    }
}

fn ensure_hpmrc() -> PathBuf {
    let mut hpmrc_path = home_dir().expect("Could not find home directory.");
    hpmrc_path.push(".hpmrc");

    if !hpmrc_path.exists() {
        let default_config = UserConfig::default();
        default_config.save_to(&hpmrc_path);
    }

    hpmrc_path
}

pub fn load_user_config() -> UserConfig {
    let hpmrc_path = ensure_hpmrc();
    UserConfig::load_from(&hpmrc_path)
}
