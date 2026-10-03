use anyhow::Result;

#[derive(Clone)]
pub struct Config {
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub anthropic_api_key: String,
    pub supabase_jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            db_host: required("DB_HOST")?,
            db_port: std::env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()?,
            db_user: required("DB_USER")?,
            db_password: required("DB_PASSWORD")?,
            db_name: std::env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string()),
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
