use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    tracing::trace!("Insert trace message here.");
    tracing::debug!("Insert debug message here.");
    tracing::info!("Insert info message here.");
    tracing::warn!("Insert warn message here.");
    tracing::error!("Insert error message here.");
    HttpResponse::Ok().finish()
}
