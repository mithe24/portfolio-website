use derive_getters::Getters;

#[derive(Getters)]
pub struct Config {
    database_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        todo!();
    }
}
