mod services;
mod models;
mod routes;

use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use routes::depth_route::get_pool_depth_price_history;
use services::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello S!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(get_pool_depth_price_history)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
