use axum::{extract::State, Json};
use serde_json::{json, Value};
use uuid::Uuid;
use std::str::FromStr;
use crate::state::AppState;

pub async fn handler(State(state): State<AppState>) -> Json<Value> {
    let test_id = Uuid::from_str("6c5d3c3b-41ef-4af6-b92f-204090176ef8").unwrap();

    // Test 1: query_as without WHERE (works)
    let all = sqlx::query_as::<_, crate::models::player::Player>("SELECT * FROM players")
        .fetch_all(&state.db).await;
    let all_debug = match &all {
        Ok(v) => format!("Ok({} rows)", v.len()),
        Err(e) => format!("Err: {e}"),
    };

    // Test 2: query_as with UUID bind
    let with_uuid = sqlx::query_as::<_, crate::models::player::Player>(
        "SELECT * FROM players WHERE id = $1"
    )
    .bind(test_id)
    .fetch_optional(&state.db).await;
    let uuid_debug = match &with_uuid {
        Ok(Some(_)) => "Ok(Some(player))".to_string(),
        Ok(None) => "Ok(None)".to_string(),
        Err(e) => format!("Err: {e}"),
    };

    // Test 3: query_as with text bind
    let with_text = sqlx::query_as::<_, crate::models::player::Player>(
        "SELECT * FROM players WHERE id::text = $1"
    )
    .bind(test_id.to_string())
    .fetch_optional(&state.db).await;
    let text_debug = match &with_text {
        Ok(Some(_)) => "Ok(Some(player))".to_string(),
        Ok(None) => "Ok(None)".to_string(),
        Err(e) => format!("Err: {e}"),
    };

    Json(json!({
        "all_players": all_debug,
        "uuid_bind": uuid_debug,
        "text_bind": text_debug,
    }))
}
