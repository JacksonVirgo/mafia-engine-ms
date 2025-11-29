use anyhow::anyhow;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Env {
    pub mafiascum_username: String,
    pub mafiascum_password: String,
    pub port: u16,
}

pub static ENV: Lazy<Env> = Lazy::new(|| {
    dotenv::dotenv().ok();
    Env {
        mafiascum_username: fetch_env("MAFIASCUM_USERNAME").unwrap(),
        mafiascum_password: fetch_env("MAFIASCUM_PASSWORD").unwrap(),
        port: fetch_env("PORT").unwrap_or("3000".into()).parse().unwrap(),
    }
});

fn fetch_env(var: &str) -> anyhow::Result<String> {
    std::env::var(var).map_err(|_| anyhow!(format!("Env var '{}' was not found", var)))
}
