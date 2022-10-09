use actix_web::{middleware, web, App, HttpServer};
use coachee::{controllers, db};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("PORT must be set");
    let pool = db::create_pool();

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(controllers::users::list)
    })
    .bind("0.0.0.0:".to_owned() + &port)?
    .run()
    .await
}
