use axum::{routing::get, Router};

#[macro_use]
extern crate diesel;
extern crate dotenvy;

mod db;
use db::models::WordPair;
use crate::diesel::RunQueryDsl;
use crate::db::schema::word_pairs::dsl::word_pairs;
use dotenvy::dotenv;
use std::env;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use crate::diesel::QueryDsl;
use axum::http::HeaderMap;
use axum::extract;
use axum::routing::post;
use crate::db::models::CreateWordPair;
use crate::db::schema::word_pairs::german;
use crate::db::schema::word_pairs::chinese;
use crate::diesel::ExpressionMethods;
use crate::db::schema::word_pairs::id;
use uuid::Uuid;
use tower_http::cors::CorsLayer;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn get_word_pairs() -> Vec<WordPair> {
    let mut connection = establish_connection();
    let vec = word_pairs.limit(1000).load::<WordPair>(&mut connection).expect("Error loading word pairs");
    return vec;
}


async fn create_wordpair(extract::Json(payload): extract::Json<CreateWordPair>){
    println!("{:?}",payload);
    let uuid = Uuid::new_v4();


    let mut connection = establish_connection();
    let result = diesel::insert_into(word_pairs)
    .values((german.eq(payload.german),chinese.eq(payload.chinese),id.eq(uuid.to_string())))
    .execute(&mut connection);

    println!("{:?}",result);
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "Server running!" }))
        .route("/word_pairs", get(|| async { 
            let pairs = get_word_pairs(); 
            let json = serde_json::to_string(&pairs).unwrap();
            let mut headers = HeaderMap::new();
            headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
            return (headers, json);
        }))
        //.route("/post_new_words", post(create_wordpair));
        .route("/post_new_words",post(|body| async { 
            create_wordpair(body).await;
            let text="{ 'foo' : 'bar' }";
            let mut headers = HeaderMap::new();
            headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
            println!("{:?}",text);
            return (headers, text);
            
         }))
         .layer(CorsLayer::permissive());
    axum::Server::bind(&"127.0.0.1:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
