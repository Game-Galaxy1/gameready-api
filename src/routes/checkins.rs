use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::{error::{AppError, AppResult}, models::checkin::CreateCheckin, state::AppState};

pub async fn create(
    State(state): State<AppState>,
    Path(player_id): Path<Uuid>,
    Json(body): Json<CreateCheckin>,
) -> AppResult<Json<serde_json::Value>> {
    let checkin = sqlx::query_as::<_, crate::models::checkin::Checkin>(
        r#"
        INSERT INTO checkins (player_id, date, sleep_hours, readiness, energy, notes)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (player_id, date) DO UPDATE SET
            sleep_hours = EXCLUDED.sleep_hours,
            readiness = EXCLUDED.readiness,
            energy = EXCLUDED.energy,
            notes = EXCLUDED.notes
        RETURNING *
        "#
    )
    .bind(player_id)
    .bind(body.date)
    .bind(body.sleep_hours)
    .bind(body.readiness)
    .bind(body.energy)
    .bind(body.notes)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(serde_json::to_value(checkin).unwrap()))
}

pub async fn get_by_date(
    State(state): State<AppState>,
    Path((player_id, date)): Path<(Uuid, chrono::NaiveDate)>,
) -> AppResult<Json<serde_json::Value>> {
    let checkin = sqlx::query_as::<_, crate::models::checkin::Checkin>(
        "SELECT * FROM checkins WHERE player_id = $1 AND date = $2"
    )
    .bind(player_id)
    .bind(date)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(serde_json::to_value(checkin).unwrap()))
}
