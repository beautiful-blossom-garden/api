use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use diesel::{QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use crate::config::database::establish_connection;
use crate::models::api::ApiResponse;
use crate::models::message::GET_DATA_SUCCESS;
use crate::models::tag::Tag;
use crate::schema::tag::dsl::tag;

pub async fn get_tags_handler() -> impl IntoResponse {
    let connection = &mut establish_connection().await;
    let results = tag.select(Tag::as_select()).load(connection).await.unwrap();

    (
        StatusCode::OK,
        Json(ApiResponse {
            message: GET_DATA_SUCCESS.to_string(),
            data: results,
        }),
    )
}
