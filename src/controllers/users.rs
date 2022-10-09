use actix_web::{web, Error, HttpResponse};

use crate::db;
use crate::models;

#[actix_web::get("/users")]
pub async fn list(pool: web::Data<db::DbPool>) -> Result<HttpResponse, Error> {
    let results = web::block(move || {
        let mut conn = pool.get()?;
        models::user::get_users(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(results) = results {
        Ok(HttpResponse::Ok().json(results))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid"));
        Ok(res)
    }
}
