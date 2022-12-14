use crate::error_handler::CustomError;
use crate::user::{User, Users};
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::Utc;
use serde_json::json;

#[post("/users")]
async fn create(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let user = Users::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
#[get("/auth")]
async fn auth(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let user = Users::basic_auth(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}
pub fn init_routes(comfig: &mut web::ServiceConfig) {
    // comfig.service(find_all);
    // comfig.service(find);
    comfig.service(create);
    comfig.service(auth);
    // comfig.service(update);
    // comfig.service(delete);
}
