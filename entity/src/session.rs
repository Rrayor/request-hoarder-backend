//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sessions_to_endpoints::Entity")]
    SessionsToEndpoints,
}

impl Related<super::sessions_to_endpoints::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SessionsToEndpoints.def()
    }
}

impl Related<super::endpoint::Entity> for Entity {
    fn to() -> RelationDef {
        super::sessions_to_endpoints::Relation::Endpoint.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::sessions_to_endpoints::Relation::Session.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
