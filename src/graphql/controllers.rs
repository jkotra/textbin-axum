use crate::entities::paste::{self, PasteRequest};
use async_graphql::{Context, Object};
use async_graphql::{EmptySubscription, Schema};
use http::header::CONTENT_TYPE;
use reqwest;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[allow(non_snake_case)]
    pub async fn getLatestPastes<'a>(&self, _ctx: &Context<'a>) -> Vec<paste::Model> {
        let resp = reqwest::get("http://127.0.0.1:8000/api/latest")
            .await
            .unwrap();
        let data: Vec<paste::Model> = serde_json::from_str(&resp.text().await.unwrap()).unwrap();
        data
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    #[allow(non_snake_case)]
    pub async fn createNewPaste<'a>(
        &self,
        _ctx: &Context<'a>,
        title: String,
        text: String,
        captcha: String,
    ) -> paste::Model {
        let post_data = serde_json::to_string(&PasteRequest {
            title,
            text,
            captcha,
        })
        .unwrap();
        let client = reqwest::Client::new();
        let resp = client
            .post("http://127.0.0.1:8000/api")
            .body(post_data)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap();
        serde_json::from_str(&resp.text().await.unwrap()).unwrap()
    }
}

pub type LatestPastesSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
