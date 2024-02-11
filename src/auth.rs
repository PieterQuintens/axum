use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    pub username: Option<String>,
    pub password: Option<String>,
}
