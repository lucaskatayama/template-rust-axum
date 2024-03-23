use axum::Router;

pub mod todo;

pub fn routes() -> Router {
    Router::new()
        .nest("/todo", todo::routes())
}
