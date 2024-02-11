use auth::LoginParams;
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

pub use self::error::{Error, Result};

mod auth;
mod error;
mod web;

// run: cargo watch -q -c -w src/ -x run
#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .merge(routes_hello())
        .merge(routes_auth())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static())
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("--> {:<12} - main_response_mapper", "RES_MAPPER");

    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/query", get(into_query_params))
        .route("/path/:name", get(into_path))
}

fn routes_auth() -> Router {
    Router::new()
        .route("/login", get(into_query_params))
        .route("/register/:name", get(into_path))
}

async fn into_query_params(Query(params): Query<LoginParams>) -> impl IntoResponse {
    println!("Current params: {:?}", params);

    let name: String = params.username.as_deref().unwrap_or("Mom").to_string();

    Html(format!("<h1>Hello, {}!</h1>", name))
}

async fn into_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("Current params: {:?}", name);

    // let name: String = path

    Html(format!("<h1>Hello, {}!</h1>", name))
}
