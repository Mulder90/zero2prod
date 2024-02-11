use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionData {
    name: String,
    email: String,
}

pub async fn subscribe(_params: web::Form<SubscriptionData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
