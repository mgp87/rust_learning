use actix_web::{HttpResponse, get, post, delete};
use actix_web::web::Path;

use crate::constants::APPLICATION_JSON;

#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: hola", "tweet 2: chao"];
    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweets)
}

#[post("/tweets")]
pub async fn create_tweet() -> HttpResponse {
    let new_tweet = "new tweet";
    HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String, )>) -> HttpResponse {
    let tweet = format!("tweet {:?}", path.0);
    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweet)
}

#[delete("/tweets/{id}")]
pub async fn remove_tweet_by_id(path: Path<(String, )>) -> HttpResponse {
    let result = "ok";
    HttpResponse::NoContent()
    .content_type(APPLICATION_JSON)
    .await
    .unwrap()
}