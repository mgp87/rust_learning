use super::schema::likes;
use crate::constants::APPLICATION_JSON;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use diesel::query_dsl::methods::FilterDsl;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "likes"]
pub struct Like {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub tweet_id: Uuid,
}
impl Like {
    pub fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id,
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn get_likes_by_tweet(
    path: Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let t_id = path.into_inner(); // tweet id desde los parametros de la url
    let t_id_uuid = Uuid::from_str(&t_id); // tweet id formateado a uuid

    if t_id_uuid.is_err() {
        println!("tweet id inválido, error: {:?}", t_id_uuid.err());
        // si no pudimos convertir a un uuid válido, asumimos que el tweet no existe.
        return HttpResponse::NotFound().await.unwrap();
    }

    let mut conn = pool
        .get()
        .expect("No pude obtener conexión a la base de datos");
    let response = list_likes(&mut conn, t_id_uuid.unwrap());

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

fn list_likes(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    t_id_uuid: Uuid,
) -> Vec<Like> {
    use crate::schema::likes::dsl::*;

    let result = likes.filter(tweet_id.eq(t_id_uuid)).load::<Like>(conn);

    match result {
        Ok(rows) => rows,
        Err(_) => vec![],
    }
}

#[post("/tweets/{id}/likes")]
pub async fn like_tweet(
    path: Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    use crate::schema::likes::dsl::*;
    let t_id = path.into_inner(); // tweet id desde los parametros de la url
    let mut conn = pool
        .get()
        .expect("No pude obtener conexión a la base de datos");

    let like = Like::new(Uuid::from_str(&t_id).unwrap());
    diesel::insert_into(likes)
        .values(&like)
        .execute(&mut conn)
        .unwrap();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like(
    path: Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    use crate::schema::likes::dsl::*;

    let t_id = path.into_inner(); // tweet id desde los parametros de la url
    let t_id_uuid = Uuid::from_str(&t_id); // tweet id formateado a uuid

    if t_id_uuid.is_err() {
        println!("tweet id inválido, error: {:?}", t_id_uuid.err());
        // si no pudimos convertir a un uuid válido, asumimos que el tweet no existe.
        return HttpResponse::NotFound().await.unwrap();
    }

    let mut conn = pool
        .get()
        .expect("No pude obtener conexión a la base de datos");
    let likes_vector = list_likes(&mut conn, t_id_uuid.unwrap());
    if likes_vector.is_empty() {
        return HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap();
    }
    let like_to_delete = likes_vector.first();
    let res =
        diesel::delete(likes.filter(tweet_id.eq(like_to_delete.unwrap().id))).execute(&mut conn);

    match res {
        Err(err) => println!("Error eliminando like, error: {}", err),
        _ => (),
    };

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
