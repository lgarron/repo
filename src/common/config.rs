use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, ErrorKind},
    path::PathBuf,
    sync::LazyLock,
};

use schemars::JsonSchema;
use serde::Deserialize;

const CONFIG_PATH: &str = "./.config/repo.json";

#[derive(Deserialize, Debug, JsonSchema)]
pub struct Config {
    pub scripts: HashMap<String, Vec<String>>,
}

// We share one lazily loaded config for the runtime of the program.
// TODO: parse this eagerly at startup if it's cheap enough.
static SHARED_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let file = match File::open(PathBuf::from(CONFIG_PATH)) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Config {
                    scripts: HashMap::default(),
                };
            }
            panic!("Config file is present, but could not be read.")
        }
    };
    serde_json::from_reader(BufReader::new(file)).unwrap()
});

// TODO: singleton
impl Config {
    pub fn get() -> &'static Self {
        &SHARED_CONFIG
    }
}
