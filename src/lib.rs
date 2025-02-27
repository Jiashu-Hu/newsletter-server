use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// We need to mark 'run' as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantations
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    /* HttpServer Explained
    This is where should the the application be listening requests? etc
    HttpServer handles all transport level concerns
    */

    /* App Explained
    What does HttpServer do when it has established a connection?
    App is where we define the application logic: routes, middleware, etc
    */
    let server = HttpServer::new(|| {
        App::new()
            /* route takes two parameters:
            path: a string, possibly templated to accommodate dynamic path segments;
            route: an instance of the Route struct
            Route combines a handler with a set of guards
            guards specify conditions that a request must satisfy in order to "match" and be passed over to handler
            web::get() is a guard that matches GET requests
            .to(health_check) is a handler function, health_check is handler
            */
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
