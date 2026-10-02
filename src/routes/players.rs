use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::{error::{AppError, AppResult}, models::player::{CreatePlayer, UpdatePlayer}, state::AppState};

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreatePlayer>,
) -> AppResult<Json<serde_json::Value>> {
    let player = sqlx::query_as::<_, crate::models::player::Player>(
        "INSERT INTO players (id, email) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET email = EXCLUDED.email RETURNING *"
    )
    .bind(body.id)
    .bind(body.email)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(serde_json::to_value(player).unwrap()))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let player = sqlx::query_as::<_, crate::models::player::Player>(
        "SELECT * FROM players WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;
    Ok(Json(serde_json::to_value(player).unwrap()))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePlayer>,
) -> AppResult<Json<serde_json::Value>> {
    let player = sqlx::query_as::<_, crate::models::player::Player>(
        r#"
        UPDATE players SET
            name = COALESCE($2, name),
            age = COALESCE($3, age),
            position = COALESCE($4, position),
            foot = COALESCE($5, foot),
            level = COALESCE($6, level),
            training_days = COALESCE($7, training_days),
            sleep_hours = COALESCE($8, sleep_hours),
            diet = COALESCE($9, diet),
            injury_history = COALESCE($10, injury_history),
            weaknesses = COALESCE($11, weaknesses),
            onboarded = COALESCE($12, onboarded),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(body.name)
    .bind(body.age)
    .bind(body.position)
    .bind(body.foot)
    .bind(body.level)
    .bind(body.training_days)
    .bind(body.sleep_hours)
    .bind(body.diet)
    .bind(body.injury_history)
    .bind(body.weaknesses)
    .bind(body.onboarded)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(serde_json::to_value(player).unwrap()))
}
