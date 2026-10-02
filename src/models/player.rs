use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Player {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub position: Option<String>,
    pub foot: Option<String>,
    pub level: Option<String>,
    pub training_days: Option<i32>,
    pub goals: Option<Vec<String>>,
    pub sleep_hours: Option<f32>,
    pub diet: Option<String>,
    pub injury_history: Option<String>,
    pub weaknesses: Option<String>,
    pub onboarded: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlayer {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlayer {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub position: Option<String>,
    pub foot: Option<String>,
    pub level: Option<String>,
    pub training_days: Option<i32>,
    pub goals: Option<Vec<String>>,
    pub sleep_hours: Option<f32>,
    pub diet: Option<String>,
    pub injury_history: Option<String>,
    pub weaknesses: Option<String>,
    pub onboarded: Option<bool>,
}
