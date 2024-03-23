use crate::{Error, Result};
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    fn new(user_id: i64) -> Self {
        Self { user_id }
    }
}

impl Ctx {
    pub fn get_user_id(&self) -> i64 {
        self.user_id
    }
}

#[async_trait]
impl<S: Sync + Send> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("Extractor {parts:?}");
        Ok(Ctx::new(123))
    }
}
