use actix_web::{HttpResponse, get, post, delete};
use actix_web::web::{Path, Data};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use diesel::{ Insertable, Queryable, RunQueryDsl };
use diesel::query_dsl::methods::{ FilterDsl, LimitDsl, OrderDsl };
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use super::schema::likes;
use crate::constants::APPLICATION_JSON;

#[table_name = "likes"]
#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Like {
    id: Uuid,
    created_at: NaiveDateTime,
    tweet_id: Uuid,
}

impl Like {
    fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            tweet_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(path: Path<(String, )>) -> HttpResponse {
    let likes = 150;
    HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn new_like(path: Path<(String, )>, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    use crate::schema::likes::dsl::*;
    let t_id = &path.0;
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let like = Like::new(Uuid::parse_str(t_id).unwrap());
    diesel::insert_into(likes)
        .values(&like)
        .execute(&mut conn)
        .expect("couldn't insert like into db");
    HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(&like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like(path: Path<(String, )>) -> HttpResponse {
    let result = "ok";
    HttpResponse::NoContent()
    .content_type(APPLICATION_JSON)
    .await
    .unwrap()
}