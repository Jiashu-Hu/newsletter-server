use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use newsletter_server::run;

#[tokio::main]
/* HttpServer::run is an asynchronous method but main cannot be an asymchronous function
    so we use tokio::main to create a runtime for us
*/
async fn main() -> std::io::Result<()> {
    run().await
}
