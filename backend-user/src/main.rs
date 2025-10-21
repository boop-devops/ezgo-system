mod app;
mod config;
mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:3000");

    HttpServer::new(|| {
        App::new()
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
