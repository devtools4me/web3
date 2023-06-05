use std::fmt;

#[derive(serde::Deserialize)]
pub struct Settings {
    //pub database: DatabaseSettings,
    pub application_port: u16
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Settings({})", self.application_port)
    }
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("conf/configuration"))?;
    settings.try_into()
}