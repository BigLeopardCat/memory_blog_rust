use dotenvy::dotenv;
use sea_orm::Database;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;

use memory_blog_rust::{create_router, AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await.expect("Failed to connect to DB");

    let app_state = AppState { db };
    let app = create_router(app_state);

    let items = vec![
        "Public Notes", "Search", "Categories", "Tag1", "Tag2", "Friends", 
        "WebInfo", "UserInfo", "SocialInfo", "Talks"
    ];
    println!("Server starting... exposing endpoints for: {:?}", items);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
