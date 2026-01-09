use axum::{Json, extract::{State, Path}};
use sea_orm::{EntityTrait, Set, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::entity::talk;
use crate::routes::AppState;
use crate::utils::ApiResponse;

#[derive(Serialize)]
pub struct TalkDto {
    #[serde(rename = "talkKey")]
    pub id: i32,
    #[serde(rename = "talkTitle")]
    pub title: String,
    pub content: String,
    #[serde(rename = "createTime")]
    pub created_at: String,
    #[serde(rename = "updateTime")]
    pub updated_at: String,
}

pub async fn list_talks(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<TalkDto>>> {
    let talks = talk::Entity::find().order_by_desc(talk::Column::CreatedAt).all(&state.db).await.unwrap_or(vec![]);
    let dtos = talks.into_iter().map(|t| TalkDto {
        id: t.id,
        title: t.title.unwrap_or_default(),
        content: t.content,
        created_at: t.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        updated_at: t.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    }).collect();
    Json(ApiResponse::success(dtos))
}

#[derive(Deserialize)]
pub struct UpsertTalk {
    #[serde(rename = "talkTitle")]
    title: String,
    content: String,
}

pub async fn create_talk(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpsertTalk>,
) -> Json<ApiResponse<String>> {
    let t = talk::ActiveModel {
        title: Set(Some(payload.title)),
        content: Set(payload.content),
        created_at: Set(chrono::Local::now().naive_local()),
        updated_at: Set(chrono::Local::now().naive_local()),
        ..Default::default()
    };
    talk::Entity::insert(t).exec(&state.db).await.unwrap();
    Json(ApiResponse::success("Created".to_string()))
}

pub async fn delete_talk(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<String>> {
    talk::Entity::delete_by_id(id).exec(&state.db).await.unwrap();
    Json(ApiResponse::success("Deleted".to_string()))
}

pub async fn update_talk(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpsertTalk>,
) -> Json<ApiResponse<String>> {
    let talk_model = talk::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .unwrap();

    if let Some(t) = talk_model {
        let mut active_model: talk::ActiveModel = t.into();
        active_model.title = Set(Some(payload.title));
        active_model.content = Set(payload.content);
        active_model.updated_at = Set(chrono::Local::now().naive_local());
        
        talk::Entity::update(active_model).exec(&state.db).await.unwrap();
        Json(ApiResponse::success("Updated".to_string()))
    } else {
        Json(ApiResponse { code: 404, message: "Talk not found".to_string(), data: String::default() })
    }
}
