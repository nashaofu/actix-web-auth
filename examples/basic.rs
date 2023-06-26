use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web, App, Error, HttpServer};

use actix_web_auth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};

async fn validator(
    req: ServiceRequest,
    credentials: Option<BasicAuth>,
    err: Option<Error>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    println!("credentials: {:?}", credentials);
    println!("err: {:?}", err);
    if let Some(err) = err {
        return Err((err, req));
    }
    if let Some(c) = credentials {
        println!("credentials: {:?} {:?}", c.user_id(), c.password());
    }
    // Ok(req)
    Err((ErrorUnauthorized(""), req))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::basic(validator);

        App::new()
            .wrap(auth)
            .route("/", web::get().to(|| async { "Hello, middleware!" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
