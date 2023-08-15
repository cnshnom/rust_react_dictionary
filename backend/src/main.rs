use axum::routing::delete;
use axum::{routing::get, Router};

#[macro_use]
extern crate diesel;
extern crate dotenvy;

mod db;
use crate::db::schema::{
    word_pairs::chinese, word_pairs::dsl::word_pairs, word_pairs::german, word_pairs::id,
};
use crate::db::models::CreateWordPair;
use crate::diesel::{
    sqlite::SqliteConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use axum::{extract::Json, extract::Path, routing::post};
use db::models::WordPair;
use dotenvy::dotenv;
use std::env;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn get_word_pairs() -> String {
    let mut connection = establish_connection();
    let pairs = word_pairs
        .limit(1000)
        .load::<WordPair>(&mut connection)
        .expect("Error loading word pairs");
    println!("Loaded {:?} word pairs from database", pairs.len());
    return serde_json::to_string(&pairs).unwrap();
}

async fn create_wordpair(Json(payload): Json<CreateWordPair>) -> String {
    let uuid = Uuid::new_v4();

    let mut connection = establish_connection();
    let _ = diesel::insert_into(word_pairs)
        .values((
            german.eq(payload.german.clone()),
            chinese.eq(payload.chinese.clone()),
            id.eq(uuid.to_string()),
        ))
        .execute(&mut connection);

    let word_pair = WordPair {
        id: uuid.to_string(),
        german: payload.german,
        chinese: payload.chinese,
    };

    let word_pair_json = serde_json::to_string(&word_pair).unwrap();
    println!("Created new word pair: {:?}", word_pair_json);
    return word_pair_json;
}

async fn delete_word_pair(Path(word_pair_id): Path<Uuid>) {
    let mut connection = establish_connection();

    let _ =
        diesel::delete(word_pairs.filter(id.eq(word_pair_id.to_string()))).execute(&mut connection);

    println!("Deleted WordPair with id {:?}", word_pair_id.to_string());
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "Server running!" }))
        .route("/word_pairs", get(get_word_pairs))
        .route("/word_pairs", post(create_wordpair))
        .route("/word_pairs/:word_pair_id", delete(delete_word_pair))
        .layer(CorsLayer::permissive());

    axum::Server::bind(&"127.0.0.1:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
