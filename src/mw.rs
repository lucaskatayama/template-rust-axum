use crate::{ctx::Ctx, Result};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
// use axum::Json;
// use serde_json::json;

pub async fn mw_extract(ctx: Result<Ctx>, req: Request, next: Next) -> Result<Response> {
    println!("{ctx:?}");
    Ok(next.run(req).await)
}
