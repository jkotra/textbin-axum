use crate::{controllers::utils::validate_captcha, dblogic::db::get_db, entities::paste::Model};
use axum::{extract::Path, extract::Query, http::StatusCode, response::IntoResponse, Json};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{
    paste::{self, PasteRequest},
    prelude::*,
};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use utoipa::IntoParams;

use log::{debug, error, info, warn};

#[derive(Deserialize, IntoParams)]
pub struct PasteDeleteQueryParams {
    key: String,
}

#[derive(Deserialize, IntoParams)]
pub struct PasteGetQueryParams {
    uuid: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

fn get_random_alphanumeric_key() -> String {
    let mut rng = thread_rng();
    let key = (0..12).map(|_| rng.sample(Alphanumeric) as char).collect();
    return key;
}

fn remove_delete_key(model: &mut Model) {
    model.set(
        paste::Column::Deletekey,
        Value::String(Some(Box::new("".to_string()))),
    );
}

#[utoipa::path(
    get,
    path = "/api?uuid={uuid}",
    params(
        PasteGetQueryParams
    ),
    responses(
        (status = 200, description = "", body = PasteRequestResponse)
    )
)]
pub async fn get_paste(params: Query<PasteGetQueryParams>) -> impl IntoResponse {
    info!("requested uuid = {}", params.uuid.clone());
    let uid = match Uuid::parse_str(&params.uuid) {
        Ok(u) => u,
        Err(err) => {
            error!("{}", err);
            return (StatusCode::NOT_FOUND, params.uuid.clone().into_response());
        }
    };

    let db = get_db().await.unwrap();

    let jerr = ErrorMessage {
        message: format!("Paste with UUID = {} not found!", uid.to_string()),
    };

    match Paste::find()
        .filter(paste::Column::Uuid.eq(uid))
        .one(&db)
        .await
    {
        Ok(p) => {
            if p.is_some() {
                let mut p = p.unwrap();
                remove_delete_key(&mut p);
                (StatusCode::OK, Json(p).into_response())
            } else {
                warn!("{} not found!", params.uuid);
                (StatusCode::NOT_FOUND, Json(jerr).into_response())
            }
        }
        Err(err) => {
            error!("{:?}", err);
            (StatusCode::NOT_FOUND, Json(jerr).into_response())
        }
    }
}

#[utoipa::path(
    post,
    path = "/api",
    request_body = PasteRequest,
    responses(
        (status = 201, description = "", body = PasteCreationResponse)
    )
)]
pub async fn post_paste(Json(req): Json<PasteRequest>) -> impl IntoResponse {
    info!("received = {:?}", req);

    match validate_captcha(req.captcha).await {
        Ok(r) => {
            if !r {
                let err = ErrorMessage {
                    message: "Google Captcha Error".to_owned(),
                };
                return (StatusCode::UNAUTHORIZED, Json(err).into_response());
            }
            r
        }
        Err(err) => {
            let err = ErrorMessage {
                message: format!("{:?}", err).to_owned(),
            };
            return (StatusCode::UNAUTHORIZED, Json(err).into_response());
        }
    };

    let db = get_db().await.unwrap();

    let timenow = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let deletekey = get_random_alphanumeric_key();

    let p = paste::ActiveModel {
        title: ActiveValue::Set(req.title.clone()),
        text: ActiveValue::Set(req.text.clone()),
        uuid: ActiveValue::Set(Uuid::new_v4()),
        date: ActiveValue::Set(timenow as i64),
        deletekey: ActiveValue::Set(deletekey),
        ..Default::default()
    };

    debug!("built entity = {:?}", p);

    let inserted = Paste::insert(p).exec(&db).await.unwrap();

    let inserted = Paste::find_by_id(inserted.last_insert_id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    (StatusCode::CREATED, Json(inserted).into_response())
}

#[utoipa::path(
    get,
    path = "/api/latest",
    responses(
        (status = 200, description = "", body = [PasteRequestResponse])
    )
)]
pub async fn get_latest() -> impl IntoResponse {
    let db = get_db().await.unwrap();

    let mut results = Paste::find()
        .cursor_by(paste::Column::Id)
        .last(5)
        .all(&db)
        .await
        .unwrap();

    for v in results.iter_mut() {
        remove_delete_key(v);
    }

    debug!("results = {:?}", results);

    (StatusCode::OK, Json(results).into_response())
}

#[utoipa::path(
    get,
    path = "/api/delete/{uuid}?key={key}",
    params(
        ("uuid" = String, Path, description = "UUID of the paste."),
        PasteDeleteQueryParams
    ),
    responses(
        (status = 200, description = "")
    )
)]
pub async fn delete_paste(
    Path(paste_uuid): Path<String>,
    params: Query<PasteDeleteQueryParams>,
) -> impl IntoResponse {
    debug!("uuid={} deletekey={}", paste_uuid, params.key);

    let db = get_db().await.unwrap();
    let uid = Uuid::parse_str(&paste_uuid).unwrap();

    match Paste::find()
        .filter(paste::Column::Uuid.eq(uid))
        .one(&db)
        .await
    {
        Ok(p) => {
            if p.is_some() {
                let p = p.unwrap();
                p.delete(&db).await.unwrap();
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
        }
        Err(err) => {
            error!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
