use actix_web::{HttpServer, web::{self, Data}, App};
use app_state::AppState;
use handlers::{handle_sign_certificate, handle_verify_certificate};
mod app_state;
mod models;
mod handlers;

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    let data = Data::new(AppState::generate().await);
    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .service(web::resource("/sign").route(web::post().to(handle_sign_certificate)))
        .service(web::resource("/verify").route(web::post().to(handle_verify_certificate)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
