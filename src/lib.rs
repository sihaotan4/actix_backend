use actix_web::{web, App, HttpResponse, HttpServer, dev::Server};
use std::net::TcpListener;

// health check response is static and does not need to use HttpRequest
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(lst: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
            .route("/health_check", web::get().to(health_check))
        )
        .listen(lst)?
        .run();
    Ok(server)
}
