use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
        pub foo: String,
}

impl Config {
        pub fn from_str(s: &str) -> Result<Self, toml::de::Error> {
                Ok(toml::from_str(s)?)
        }
}

#[derive(Debug)]
pub struct ConfigError {
        io_error: Option<std::io::Error>,
        toml_error: Option<toml::de::Error>,
}

pub async fn load(f: &str) -> Result<Config, ConfigError> {
        let file = tokio::fs::read_to_string(f).await;

        let cfg = match file {
            Ok(s) => Config::from_str(&s),
            Err(e) => {
                    return Err(ConfigError { io_error: Some(e), toml_error: None });
            }
        };

        match cfg {
            Ok(c) => Ok(c),
            Err(e) => {
                    Err(ConfigError { toml_error: Some(e), io_error: None })
            }
        }
}
