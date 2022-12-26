//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Eq)]
#[sea_orm(table_name = "paste")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub date: i64,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub title: String,
    pub text: String,
    pub deletekey: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/* Request */

#[derive(Serialize, Deserialize, Debug)]
pub struct PasteRequest {
    pub title: String,
    pub text: String,
    pub captcha: String,
}
