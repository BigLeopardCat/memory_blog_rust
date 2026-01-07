use axum::{Json, extract::State};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use std::sync::Arc;
use crate::entity::user;
use crate::routes::AppState;
use crate::utils::ApiResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Json<ApiResponse<String>> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(payload.username))
        .one(&state.db)
        .await
        .unwrap_or(None);

    if let Some(u) = user {
        if u.password == payload.password {
            // Generate a token. For now, a simple mock token or username-based.
            // In production, use JWT.
            let token = format!("mock-token-for-{}", u.id);
            return Json(ApiResponse::success(token));
        }
    }

    Json(ApiResponse::error("Invalid credentials"))
}
