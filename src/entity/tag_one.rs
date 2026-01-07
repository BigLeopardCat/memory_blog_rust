use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "tag_one")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub level: Option<i32>,
    pub color: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tag_two::Entity")]
    TagTwo,
}

impl Related<super::tag_two::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TagTwo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
