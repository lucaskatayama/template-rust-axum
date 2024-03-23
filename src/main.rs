use axum::response::Response;
use axum::{middleware, routing::get_service, Router};
use envconfig::Envconfig;
pub use error::{Error, Result};
use tower_http::services::ServeDir;

mod api;
mod ctx;
mod error;
mod mw;

fn static_router() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./web")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", " RES_MAPPER");
    res
}

#[derive(Envconfig)]
struct Config {
    #[envconfig(default = "127.0.0.1")]
    host: String,
    #[envconfig(default = "8080")]
    port: String,
}

#[tokio::main]
async fn main() {
    let cfg = Config::init_from_env().unwrap();

    // build our application with a single route
    let app = Router::new()
        .nest("/api", api::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn(mw::mw_extract))
        .fallback_service(static_router());

    let addr = format!("{}:{}", cfg.host, cfg.port).to_string();
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
