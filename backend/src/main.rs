use axum::routing::delete;
use axum::{routing::get, Router};

#[macro_use]
extern crate diesel;
extern crate dotenvy;

mod db;
use crate::db::models::CreateWordPair;
use crate::db::models::DeleteWordPair;
use crate::db::schema::word_pairs::chinese;
use crate::db::schema::word_pairs::dsl::word_pairs;
use crate::db::schema::word_pairs::german;
use crate::db::schema::word_pairs::id;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use axum::extract;
use axum::http::HeaderMap;
use axum::routing::post;
use db::models::WordPair;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
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

fn get_word_pairs() -> Vec<WordPair> {
    let mut connection = establish_connection();
    let vec = word_pairs
        .limit(1000)
        .load::<WordPair>(&mut connection)
        .expect("Error loading word pairs");
    return vec;
}

async fn create_wordpair(extract::Json(payload): extract::Json<CreateWordPair>) -> WordPair {
    let uuid = Uuid::new_v4();

    let mut connection = establish_connection();
    let _ = diesel::insert_into(word_pairs)
        .values((
            german.eq(payload.german.clone()),
            chinese.eq(payload.chinese.clone()),
            id.eq(uuid.to_string()),
        ))
        .execute(&mut connection);

    return WordPair {
        id: uuid.to_string(),
        german: payload.german,
        chinese: payload.chinese
    };
}

async fn delete_wordpair(extract::Json(payload): extract::Json<DeleteWordPair>){
    let mut connection = establish_connection();
    println!("{:?}",payload.id);

    let _ =diesel::delete(word_pairs.filter(id.eq(payload.id))).execute(&mut connection);
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "Server running!" }))
        .route(
            "/word_pairs",
            get(|| async {
                let pairs = get_word_pairs();
                let json = serde_json::to_string(&pairs).unwrap();
                let mut headers = HeaderMap::new();
                headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
                return (headers, json);
            }),
        )
        .route(
            "/post_new_words",
            post(|body| async {
                let word_pair = create_wordpair(body).await;
                let mut headers = HeaderMap::new();
                headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
                let text = serde_json::to_string(&word_pair).unwrap();
                println!("{:?}", text);
                return (headers, text);
            }),
        )
        .route("/delete", delete(|body|async{
            delete_wordpair(body).await;
        }))
        .layer(CorsLayer::permissive());
    
    axum::Server::bind(&"127.0.0.1:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
