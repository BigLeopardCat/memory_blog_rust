use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    // New fields
    pub introduce: Option<String>,
    pub path_name: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::note::Entity")]
    Note,
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
