use crate::{ctx, Result};
use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(list_files))
        .route("/:id", get(get_file))
}

async fn list_files(ctx: ctx::Ctx) -> Result<Json<Value>> {
    println!("->> CTX on list_files: {}", ctx.get_user_id());
    Ok(Json(json!(vec!["123"])))
}

async fn get_file(Path(id): Path<String>) -> Result<Json<Value>> {
    Ok(Json(json!({"hello": id})))
}
