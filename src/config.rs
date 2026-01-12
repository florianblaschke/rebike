use serde::Deserialize;

#[derive(Deserialize)]
struct ATS {
    endpoint: String,
}

#[derive(Deserialize)]
struct Database {
    uri: String,
}

#[derive(Deserialize)]
struct Configuration {
    ats: ATS,
    db: Database,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let configuration_builder = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("__")
                .separator("_"),
        )
        .build()?;

    configuration_builder.try_deserialize::<Configuration>()
}
