use crate::{
    models::{
        api::ApiResponse,
        auth::AuthLoginRequest,
        message::{LOGIN_FAILED, LOGIN_SUCCESS},
    },
    services,
};
use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn login_handler(payload: Json<AuthLoginRequest>) -> impl IntoResponse {
    let service = services::auth::AuthService::new();
    let result = service.login(payload.username.clone(), payload.password.clone());

    match result {
        Ok(token) => {
            let response = ApiResponse {
                message: LOGIN_SUCCESS.to_string(),
                data: token,
            };
            (StatusCode::OK, Json(response))
        }
        Err(_) => {
            let response = ApiResponse {
                message: LOGIN_FAILED.to_string(),
                data: "".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(response))
        }
    }
}
