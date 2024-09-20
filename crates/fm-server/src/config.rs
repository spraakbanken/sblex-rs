use config::{Config, Environment};
use serde::Deserialize;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    #[serde(alias = "MORPHOLOGY_PATH")]
    pub morphology_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let s = Config::builder()
            .add_source(Environment::default())
            .add_source(Environment::with_prefix("FM_SERVER").prefix_separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
