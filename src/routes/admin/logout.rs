use actix_web::HttpResponse;

use crate::utils::see_other;

pub async fn log_out() -> Result<HttpResponse, actix_web::Error> {
    Ok(see_other("/login"))
}