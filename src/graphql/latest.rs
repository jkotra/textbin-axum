use crate::dblogic::db::get_db;
use crate::entities::paste;
use crate::entities::prelude::*;
use async_graphql::{Context, Object};
use sea_orm::*;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[allow(non_snake_case)]
    pub async fn getLatestPastes<'a>(&self, _ctx: &Context<'a>) -> Vec<paste::Model> {
        let db = get_db().await;
        Paste::find()
            .cursor_by(paste::Column::Id)
            .last(5)
            .all(&db.unwrap())
            .await
            .unwrap()
    }
}

pub type LatestPastesSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
