pub mod controller;
pub mod models;
pub mod schema;

use axum::{
    routing::{get, post},
    Router,
};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::controller::{create_reading, events_handler};

// This embeds the migrations into the application binary
// the migration path is relative to the `CARGO_MANIFEST_DIR`
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    //env_logger::init();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ambi_rs=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let static_files_service = ServeDir::new(static_dir).append_index_html_on_directories(true);

    // build our application with some routes
    let app = Router::new()
        .fallback_service(static_files_service)
        .route("/events", get(events_handler))
        .route("/api/readings/add", post(create_reading))
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    //let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
    //axum::serve(listener, app).await.unwrap();
}
