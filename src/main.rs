#![allow(unused)]
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use anyhow::Result;

mod helpers;
mod request_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(|| {
      App::new()
        .service(
          web::resource("/v1/chat/completions")
            .route(web::post().to(request_handler::chat_completion))
            .route(web::route().to(request_handler::invalid_method))
          )
          .default_service(web::route().to(request_handler::invalid_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
