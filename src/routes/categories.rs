use axum::{Json, extract::{State, Path}};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, DeleteResult, PaginatorTrait, ModelTrait, QuerySelect, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::entity::{category, note};
use crate::routes::AppState;
use crate::utils::ApiResponse;

#[derive(Serialize)]
pub struct CategoryDto {
    #[serde(rename = "categoryKey")]
    pub key: i32, // Frontend CategoriesType says 'key' AND 'categoryKey'?
    // "key: number; categoryKey: string | undefined;"
    #[serde(rename = "key")]
    pub id: i32, 

    #[serde(rename = "categoryTitle")]
    pub name: String,
    #[serde(rename = "pathName")]
    pub path_name: String,
    pub introduce: String,
    pub icon: String,
    pub color: String,
    #[serde(rename = "noteCount")]
    pub note_count: usize,
}

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<CategoryDto>>> {
    let categories = category::Entity::find().all(&state.db).await.unwrap_or(vec![]);
    
    let mut dtos = Vec::new();
    for cat in categories {
        let count = note::Entity::find()
            .filter(note::Column::CategoryId.eq(cat.id))
            .count(&state.db)
            .await
            .unwrap_or(0);
            
        dtos.push(CategoryDto {
            id: cat.id,
            key: cat.id,
            name: cat.name,
            path_name: cat.path_name.unwrap_or_default(),
            introduce: cat.introduce.unwrap_or_default(),
            icon: cat.icon.unwrap_or_default(),
            color: cat.color.unwrap_or_default(),
            note_count: count as usize,
        });
    }

    Json(ApiResponse::success(dtos))
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    #[serde(rename = "categoryTitle")]
    pub name: String,
    #[serde(rename = "pathName")]
    pub path_name: String,
    pub introduce: String,
    pub icon: String,
    pub color: String,
}

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Json<ApiResponse<String>> {
    let new_cat = category::ActiveModel {
        name: Set(payload.name),
        path_name: Set(Some(payload.path_name)),
        introduce: Set(Some(payload.introduce)),
        icon: Set(Some(payload.icon)),
        color: Set(Some(payload.color)),
        ..Default::default()
    };

    match new_cat.insert(&state.db).await {
        Ok(_) => Json(ApiResponse::success("Category created".to_string())),
        Err(e) => Json(ApiResponse::error(&format!("Error: {}", e))),
    }
}

pub async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Json<ApiResponse<String>> {
    let cat = category::Entity::find_by_id(id).one(&state.db).await.unwrap_or(None);
    if let Some(c) = cat {
        let mut active: category::ActiveModel = c.into();
        active.name = Set(payload.name);
        active.path_name = Set(Some(payload.path_name));
        active.introduce = Set(Some(payload.introduce));
        active.icon = Set(Some(payload.icon));
        active.color = Set(Some(payload.color));
        
        match active.update(&state.db).await {
            Ok(_) => Json(ApiResponse::success("Updated".to_string())),
            Err(e) => Json(ApiResponse::error(&format!("Error: {}", e))),
        }
    } else {
        Json(ApiResponse::error("Not found"))
    }
}

pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    Json(keys): Json<Vec<i32>>,
) -> Json<ApiResponse<String>> {
    match category::Entity::delete_many()
        .filter(category::Column::Id.is_in(keys))
        .exec(&state.db)
        .await {
        Ok(_) => Json(ApiResponse::success("Deleted".to_string())),
        Err(e) => Json(ApiResponse::error(&format!("Error: {}", e))),
    }
}
