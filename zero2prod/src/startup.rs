use crate::routes::{health_check, subscribe};
use actix_web::{dev, web, web::Data, App, HttpServer};
use std::{io, net};
use tracing_actix_web::TracingLogger;

pub fn run(listener: net::TcpListener, db_pool: sqlx::PgPool) -> Result<dev::Server, io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
