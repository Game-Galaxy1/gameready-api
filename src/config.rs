use anyhow::Result;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub anthropic_api_key: String,
    pub supabase_jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            database_url: required("DATABASE_URL")?,
            anthropic_api_key: required("ANTHROPIC_API_KEY")?,
            supabase_jwt_secret: required("SUPABASE_JWT_SECRET")?,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
        })
    }
}

fn required(key: &str) -> Result<String> {
    std::env::var(key).map_err(|_| anyhow::anyhow!("Missing required env var: {}", key))
}
