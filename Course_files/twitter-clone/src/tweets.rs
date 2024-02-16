use super::schema::tweets;
use crate::constants::APPLICATION_JSON;
use actix_web::web::{Data, Path};
use actix_web::{get, post, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

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

    let mut conn = pool
        .get()
        .expect(" No pude obtener conexión a la base de datos");
    let result = tweets
        .order(created_at.desc())
        .limit(10) // sería bueno parametrizar este valor
        .load::<Tweet>(&mut conn);

    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

#[post("/tweets")]
pub async fn create_tweet(
    req_body: String,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let nuevo_tweet = Tweet::new(req_body);
    let mut conn = pool
        .get()
        .expect(" No pude obtener conexión a la base de datos");

    diesel::insert_into(tweets::table)
        .values(&nuevo_tweet)
        .execute(&mut conn)
        .expect("Error al insertar tweet");

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&nuevo_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(
    path: Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    use crate::schema::tweets::dsl::*;

    let t_id = path.into_inner(); // tweet id desde los parametros de la url

    let mut conn = pool
        .get()
        .expect(" No pude obtener conexión a la base de datos"); // tweet id desde los parametros de la url

    let t_id_uuid = Uuid::from_str(&t_id); // tweet id formateado a uuid
    if t_id_uuid.is_err() {
        println!("tweet id inválido, error: {:?}", t_id_uuid.err());
        // si no pudimos convertir a un uuid válido, asumimos que el tweet no existe.
        return HttpResponse::NotFound().await.unwrap();
    }

    let result = tweets
        .filter(id.eq(t_id_uuid.unwrap()))
        .load::<Tweet>(&mut conn);

    match result {
        Ok(rows) => match rows.first() {
            Some(tweet) => HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(tweet),
            _ => HttpResponse::NotFound().await.unwrap(),
        },
        Err(_) => HttpResponse::NotFound().await.unwrap(),
    }
}
