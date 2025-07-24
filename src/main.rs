use axum::Router;

use api::routers::v1::route::v1_router;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().merge(v1_router());

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
