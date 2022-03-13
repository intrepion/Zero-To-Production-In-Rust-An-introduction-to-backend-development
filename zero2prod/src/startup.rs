use crate::{
    configuration::Settings,
    email_client::{self, EmailClient},
    routes::{health_check, subscribe},
};
use actix_web::{dev, web, web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{io, net};
use tracing_actix_web::TracingLogger;

pub async fn build(configuration: Settings) -> Result<dev::Server, io::Error> {
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = net::TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)
}

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
