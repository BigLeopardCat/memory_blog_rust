use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename = "imageKey")] 
    pub image_key: i32,
    #[serde(rename = "imageUrl")] 
    pub image_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
