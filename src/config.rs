use serde::Deserialize;

#[derive(Deserialize)]
pub struct ATS {
    pub endpoint: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub ats: ATS,
    pub db: Database,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let configuration_builder = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    configuration_builder.try_deserialize::<Configuration>()
}
