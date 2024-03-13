use std::collections::HashMap;

use config::{Config, Environment};

use super::*;

#[test]
fn settings_pick_up_unprefixed_values() -> Result<(), config::ConfigError> {
    let source = Environment::default().source(Some({
        let mut env = HashMap::new();
        env.insert("morphology_path".into(), "assets/testing/saldo.lex".into());
        env
    }));

    let settings: Settings = Config::builder()
        .add_source(source)
        .build()?
        .try_deserialize()?;
    assert_eq!(settings.morphology_path, "assets/testing/saldo.lex");
    Ok(())
}
#[test]
fn settings_pick_up_prefixed_values() -> Result<(), config::ConfigError> {
    let source = Environment::with_prefix("FM_SERVER")
        .prefix_separator("__")
        .source(Some({
            let mut env = HashMap::new();
            env.insert(
                "FM_SERVER__MORPHOLOGY_PATH".into(),
                "assets/testing/saldo.lex".into(),
            );
            env
        }));

    let settings: Settings = Config::builder()
        .add_source(source)
        .build()?
        .try_deserialize()?;
    assert_eq!(settings.morphology_path, "assets/testing/saldo.lex");
    Ok(())
}
