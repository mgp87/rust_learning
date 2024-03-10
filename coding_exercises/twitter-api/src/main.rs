#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use std::env;
use dotenv::dotenv;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
mod tweets;
mod likes;
mod schema;
mod constants;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<diesel::PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .service(tweets::get_tweets)
        .service(tweets::create_tweet)
        .service(tweets::get_tweet_by_id)
        .service(likes::get_likes_by_tweet)
        .service(likes::new_like)
        .service(likes::remove_like)
        .service(tweets::remove_tweet_by_id)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
