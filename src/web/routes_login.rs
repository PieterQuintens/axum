use crate::error::{Error, Result};
use crate::web;
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};


pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("--> {:<12} - api_login", "HANDLER");

    // TODO: Impl real db/auth logic
    if payload.username != "pieter" || payload.password != "dev" {
        return Err(Error::LoginFail);
    }

    // TODO: Implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN,"user-1.exp.sign"));

    let body = Json(json!({
        "result":{
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
