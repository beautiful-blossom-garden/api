use axum::{
    Router,
    routing::{get, post},
};

use crate::routers::v1::auth::login_handler;
use crate::routers::v1::tag::get_tags_handler;

pub fn v1_router() -> Router {
    Router::new()
        .route("/v1/auth/login", post(login_handler))
        .route("/v1/tags", get(get_tags_handler))
}
