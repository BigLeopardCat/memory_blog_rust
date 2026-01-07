use axum::{Json, extract::{State, Query, Path}};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Condition, ActiveModelTrait, ActiveValue, Set, ModelTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::entity::{note, category};
use crate::routes::AppState;
use crate::utils::ApiResponse;

#[derive(Deserialize)]
pub struct NoteQuery {
    pub category_id: Option<i32>,
    pub page: Option<u64>,
}

#[derive(Serialize)]
pub struct NoteDto {
    #[serde(rename = "noteKey")]
    pub id: i32,
    #[serde(rename = "key")]
    pub key: i32, 

    #[serde(rename = "noteTitle")]
    pub title: String,
    
    #[serde(rename = "noteContent")]
    pub content: String,
    #[serde(rename = "content")] 
    pub content_raw: String, 
    
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "cover")]
    pub cover: String,
    
    #[serde(rename = "createTime")]
    pub created_at: String,
    #[serde(rename = "updateTime")]
    pub updated_at: String,
    
    #[serde(rename = "isTop")]
    pub is_top: i32,
    pub status: String,
    
    #[serde(rename = "noteCategory")]
    pub category_name: Option<String>, 
    #[serde(rename = "categoryTitle")]
    pub category_title: Option<String>,
    
    pub is_public: bool,
    #[serde(rename = "noteTags")]
    pub tags: Vec<i32>, 
}

pub async fn list_public_notes(
    State(state): State<Arc<AppState>>,
    Query(query): Query<NoteQuery>,
) -> Json<ApiResponse<Vec<NoteDto>>> {
    let mut condition = Condition::all();

    if let Some(cat_id) = query.category_id {
        condition = condition.add(note::Column::CategoryId.eq(cat_id));
    }

    let notes = note::Entity::find()
        .filter(condition)
        .order_by_desc(note::Column::CreatedAt)
        .find_with_related(category::Entity)
        .all(&state.db)
        .await
        .unwrap_or(vec![]);

    let dtos = notes.into_iter().map(|(n, cats)| {
        let cat = cats.into_iter().next();
        let cat_name = cat.map(|c| c.name);
        NoteDto {
            id: n.id,
            key: n.id,
            title: n.title.clone(),
            content: n.content.clone(),
            content_raw: n.content,
            description: n.description.unwrap_or_default(),
            cover: n.cover.unwrap_or_default(),
            created_at: n.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
             updated_at: n.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            is_top: n.is_top.unwrap_or(0),
            status: n.status.unwrap_or("published".to_string()),
            category_name: cat_name.clone(),
            category_title: cat_name,
            is_public: n.is_public,
            tags: vec![],
        }
    }).collect();

    Json(ApiResponse::success(dtos))
}

#[derive(Deserialize)]
pub struct SearchRequest {
    keyword: String,
}

pub async fn search_notes(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchRequest>,
) -> Json<ApiResponse<Vec<NoteDto>>> {
     let notes = note::Entity::find()
        .filter(
            Condition::any()
                .add(note::Column::Title.contains(&payload.keyword))
                .add(note::Column::Content.contains(&payload.keyword))
        )
        .find_with_related(category::Entity)
        .all(&state.db)
        .await
        .unwrap_or(vec![]);

     let dtos = notes.into_iter().map(|(n, cats)| {
        let cat = cats.into_iter().next();
        let cat_name = cat.map(|c| c.name);
        NoteDto {
            id: n.id,
            key: n.id,
            title: n.title.clone(),
            content: n.content.clone(),
            content_raw: n.content,
             description: n.description.unwrap_or_default(),
            cover: n.cover.unwrap_or_default(),
             created_at: n.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
             updated_at: n.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            is_top: n.is_top.unwrap_or(0),
            status: n.status.unwrap_or("published".to_string()),
            category_name: cat_name.clone(),
            category_title: cat_name,
            is_public: n.is_public,
             tags: vec![],
        }
    }).collect();

    Json(ApiResponse::success(dtos))
}

#[derive(Deserialize)]
pub struct UpsertNoteRequest {
    #[serde(rename = "noteTitle")]
    pub title: String,
    #[serde(rename = "noteContent")]
    pub content: String,
    
    #[serde(rename = "noteCategory")]
    pub category_id: Option<i32>, 
    
    #[serde(rename = "isTop")]
    pub is_top: Option<i32>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub cover: Option<String>,
    
    #[serde(rename = "noteTags")]
    pub tags: Option<String>,

    #[serde(default)]
    pub is_public: bool,
}

pub async fn get_top_notes(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<NoteDto>>> {
    let notes = note::Entity::find()
        .filter(note::Column::IsTop.eq(1))
        .find_with_related(category::Entity)
        .all(&state.db)
        .await
        .unwrap_or(vec![]);

     let dtos = notes.into_iter().map(|(n, cats)| {
        let cat = cats.into_iter().next();
        let cat_name = cat.map(|c| c.name);
        NoteDto {
            id: n.id,
            key: n.id,
            title: n.title.clone(),
            content: n.content.clone(),
            content_raw: n.content,
             description: n.description.unwrap_or_default(),
            cover: n.cover.unwrap_or_default(),
             created_at: n.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
             updated_at: n.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            is_top: n.is_top.unwrap_or(0),
            status: n.status.unwrap_or("published".to_string()),
            category_name: cat_name.clone(),
            category_title: cat_name,
            is_public: n.is_public,
             tags: vec![],
        }
    }).collect();

    Json(ApiResponse::success(dtos))
}

pub async fn create_note(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpsertNoteRequest>,
) -> Json<ApiResponse<String>> {
    let new_note = note::ActiveModel {
        title: Set(payload.title),
        content: Set(payload.content),
        is_public: Set(payload.is_public),
        category_id: Set(payload.category_id),
        description: Set(payload.description),
        cover: Set(payload.cover),
        is_top: Set(payload.is_top),
        status: Set(payload.status),
        created_at: Set(chrono::Local::now().naive_local()),
        updated_at: Set(chrono::Local::now().naive_local()),
        ..Default::default()
    };

    match new_note.insert(&state.db).await {
        Ok(_) => Json(ApiResponse::success("Note created successfully".to_string())),
        Err(e) => Json(ApiResponse::error(&format!("Error: {}", e))),
    }
}

pub async fn update_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpsertNoteRequest>,
) -> Json<ApiResponse<String>> {
    let note_data = note::Entity::find_by_id(id).one(&state.db).await.unwrap_or(None);
    
    if let Some(n) = note_data {
        let mut active_model: note::ActiveModel = n.into();
        active_model.title = Set(payload.title);
        active_model.content = Set(payload.content);
        active_model.category_id = Set(payload.category_id);
        active_model.description = Set(payload.description);
        active_model.cover = Set(payload.cover);
        active_model.is_top = Set(payload.is_top);
        active_model.status = Set(payload.status);
        active_model.updated_at = Set(chrono::Local::now().naive_local());
        
        match active_model.update(&state.db).await {
            Ok(_) => Json(ApiResponse::success("Note updated successfully".to_string())),
            Err(e) => Json(ApiResponse::error(&format!("Error: {}", e))),
        }
    } else {
         Json(ApiResponse::error("Note not found"))
    }
}

pub async fn delete_note(
    State(state): State<Arc<AppState>>,
    Json(keys): Json<Vec<i32>>,
) -> Json<ApiResponse<String>> {
    match note::Entity::delete_many()
        .filter(note::Column::Id.is_in(keys))
        .exec(&state.db)
        .await {
        Ok(_) => Json(ApiResponse::success("Deleted".to_string())),
        Err(e) => Json(ApiResponse::error(&format!("Error: {}", e))),
    }
}

pub async fn get_note_detail(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Json<ApiResponse<Option<NoteDto>>> {
    let res = note::Entity::find_by_id(id)
        .find_with_related(category::Entity)
        .all(&state.db)
        .await
        .unwrap_or(vec![]);

    let dto = res.into_iter().next().map(|(n, cats)| {
         let cat = cats.into_iter().next();
         let cat_name = cat.map(|c| c.name);
         NoteDto {
            id: n.id,
            key: n.id,
            title: n.title.clone(),
            content: n.content.clone(),
            content_raw: n.content,
             description: n.description.unwrap_or_default(),
            cover: n.cover.unwrap_or_default(),
             created_at: n.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
             updated_at: n.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            is_top: n.is_top.unwrap_or(0),
            status: n.status.unwrap_or("published".to_string()),
            category_name: cat_name.clone(),
            category_title: cat_name,
            is_public: n.is_public,
             tags: vec![],
         }
    });

    Json(ApiResponse::success(dto))
}
