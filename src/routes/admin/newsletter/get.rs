use actix_web::HttpResponse;

pub async fn publish_newsletter_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}