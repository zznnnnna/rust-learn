use actix_web::{delete, get, HttpResponse, post, put, web};
use serde_json::json;

use crate::employees::{Employee, Employees};
use crate::error_handler::CustomError;

#[get("/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let employees = web::block(|| Employees::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(employees))
}

#[get("/employees/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
async fn create(employee: web::Json<Employee>) -> Result<HttpResponse, CustomError> {
    println!("employees is {:?}",employee);
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
async fn update(id: web::Path<i32>, employee: web::Json<Employee>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted:":employee})))
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}