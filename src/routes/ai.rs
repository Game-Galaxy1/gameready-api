use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
    Json,
};
use uuid::Uuid;
use crate::{error::{AppError, AppResult}, models::ai::ChatRequest, state::AppState};

pub async fn chat(
    State(state): State<AppState>,
    Path(player_id): Path<Uuid>,
    Json(body): Json<ChatRequest>,
) -> AppResult<Response> {
    // Load player context
    let player = sqlx::query_as::<_, crate::models::player::Player>(
        "SELECT * FROM players WHERE id = $1"
    )
    .bind(player_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    // Load today's check-in if it exists
    let today = chrono::Utc::now().date_naive();
    let checkin = sqlx::query_as::<_, crate::models::checkin::Checkin>(
        "SELECT * FROM checkins WHERE player_id = $1 AND date = $2"
    )
    .bind(player_id)
    .bind(today)
    .fetch_optional(&state.db)
    .await?;

    // Build system prompt with full player context
    let system = build_system_prompt(&player, checkin.as_ref());

    // Call Anthropic API with streaming
    let client = reqwest::Client::new();
    let anthropic_body = serde_json::json!({
        "model": "claude-opus-5-5",
        "max_tokens": 1024,
        "system": system,
        "messages": body.messages,
        "stream": true
    });

    let upstream = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &state.config.anthropic_api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&anthropic_body)
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let stream = upstream.bytes_stream();
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .header("content-type", "text/event-stream")
        .header("cache-control", "no-cache")
        .body(body)
        .unwrap())
}

fn build_system_prompt(
    player: &crate::models::player::Player,
    checkin: Option<&crate::models::checkin::Checkin>,
) -> String {
    let mut prompt = format!(
        "You are GameReady AI — a personal soccer coach. You speak directly, confidently and like a real coach who knows this player well. Keep responses concise and actionable.\n\n\
        PLAYER PROFILE:\n\
        - Name: {}\n\
        - Age: {}\n\
        - Position: {}\n\
        - Dominant foot: {}\n\
        - Level: {}\n",
        player.name.as_deref().unwrap_or("Unknown"),
        player.age.map(|a| a.to_string()).unwrap_or_else(|| "Unknown".to_string()),
        player.position.as_deref().unwrap_or("Unknown"),
        player.foot.as_deref().unwrap_or("Unknown"),
        player.level.as_deref().unwrap_or("Unknown"),
    );

    if let Some(w) = &player.weaknesses {
        prompt.push_str(&format!("- Areas to improve: {w}\n"));
    }
    if let Some(i) = &player.injury_history {
        prompt.push_str(&format!("- Injury history: {i}\n"));
    }

    if let Some(c) = checkin {
        prompt.push_str(&format!(
            "\nTODAY'S CHECK-IN:\n\
            - Sleep: {}hrs\n\
            - Readiness: {}/5\n",
            c.sleep_hours,
            c.readiness,
        ));
    }

    prompt.push_str("\nAlways tailor your advice to this specific player. Never give generic advice.");
    prompt
}
