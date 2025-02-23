use crate::absence::{Absence, Absences};
use crate::error_handler::CustomError;
use crate::TokenClaims;
use actix_web::web::ReqData;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::Utc;
use serde_json::json;

// #[get("/absences")]
// pub async fn find_all(req_user: Option<ReqData<TokenClaims>>) -> Result<HttpResponse, CustomError> {
//     let employees = Absences::find_all(req_user.unwrap().code.to_string())?;
//     Ok(HttpResponse::Ok().json(employees))
// }
#[get("/absences/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = Absences::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/absences")]
async fn create(
    employee: web::Json<Absence>,
    req_user: Option<ReqData<TokenClaims>>,
) -> Result<HttpResponse, CustomError> {
    println!("{}", req_user.clone().unwrap().exp);
    let employee = Absences::create(employee.into_inner(), req_user.unwrap().sub.to_string())?;
    Ok(HttpResponse::Ok().json(employee))
}
#[put("/absences/{id}")]
async fn update(
    id: web::Path<i32>,
    employee: web::Json<Absence>,
) -> Result<HttpResponse, CustomError> {
    let mut a = employee;
    a.check_out_time = Some(Utc::now());
    let employee = Absences::update(id.into_inner(), a.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/absences/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Absences::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    // comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}

// base 64 string untuk image ke string--rust
