use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RateLimitResponse {
    pub message: String,
    pub retry_after: f64,
    pub global: bool,
}
