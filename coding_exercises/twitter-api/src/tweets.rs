use actix_web::{HttpResponse, get, post, delete};
use actix_web::web::{Path, Data};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use diesel::{ Insertable, Queryable, RunQueryDsl, ExpressionMethods };
use diesel::query_dsl::methods::{ FilterDsl, LimitDsl, OrderDsl };
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use super::schema::tweets;
use crate::constants::APPLICATION_JSON;

#[diesel(table_name = tweets)]
#[derive(Queryable, Insertable, Serialize, Deserialize)]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    use crate::schema::tweets::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let result = tweets.order(created_at).limit(10).load::<Tweet>(&mut conn);
    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![], // empty vec on error
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

#[post("/tweets")]
pub async fn create_tweet(req_body: String, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    let new_tweet = Tweet::new(req_body);
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    diesel::insert_into(tweets::table)
        .values(&new_tweet)
        .execute(&mut conn)
        .expect("couldn't insert tweet into db");
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