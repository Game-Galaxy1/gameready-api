use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Checkin {
    pub id: Uuid,
    pub player_id: Uuid,
    pub date: NaiveDate,
    pub sleep_hours: f32,
    pub readiness: i32,
    pub energy: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckin {
    pub date: NaiveDate,
    pub sleep_hours: f32,
    pub readiness: i32,
    pub energy: Option<i32>,
    pub notes: Option<String>,
}
