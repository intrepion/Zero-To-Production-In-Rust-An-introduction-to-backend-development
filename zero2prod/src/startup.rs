use crate::{
    email_client,
    routes::{health_check, subscribe},
};
use actix_web::{dev, web, web::Data, App, HttpServer};
use sqlx::PgPool;
use std::{io, net};
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: net::TcpListener,
    db_pool: PgPool,
    email_client: email_client::EmailClient,
) -> Result<dev::Server, io::Error> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
