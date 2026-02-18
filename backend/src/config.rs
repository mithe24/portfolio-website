use derive_getters::Getters;

#[derive(Getters)]
pub struct Config {
    database_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self { database_url }
    }
}
