use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "tag_two")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub level: Option<i32>,
    pub color: Option<String>,
    pub tag_one_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tag_one::Entity",
        from = "Column::TagOneId",
        to = "super::tag_one::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    TagOne,
}

impl Related<super::tag_one::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TagOne.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
