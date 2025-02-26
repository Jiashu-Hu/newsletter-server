use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
/* HttpServer::run is an asynchronous method but main cannot be an asymchronous function
    so we use tokio::main to create a runtime for us
*/
async fn main() -> std::io::Result<()> {
    /* HttpServer Explained
    This is where should the the application be listening requests? etc
    HttpServer handles all transport level concerns
    */

    /* App Explained
    What does HttpServer do when it has established a connection?
    App is where we define the application logic: routes, middleware, etc
    */
    HttpServer::new(|| {
        App::new()
            /* route takes two parameters:
            path: a string, possibly templated to accommodate dynamic path segments;
            route: an instance of the Route struct
            Route combines a handler with a set of guards
            guards specify conditions that a request must satisfy in order to "match" and be passed over to handler
            web::get() is a guard that matches GET requests
            .to(greet) is a handler function, greet is handler
            */
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
