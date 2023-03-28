use actix_web::{http,HttpRequest, HttpResponse, web};

use crate::repository::{hr};
use crate::errors::MyError;
use crate::models::Hr;
pub async fn gets(req:HttpRequest) -> Result<HttpResponse,MyError>{
    if let Some(header) = req.headers().get(http::header::CONTENT_TYPE){
        if let Ok(_s) = header.to_str(){
            let emps = hr::gets().await?;
            return Ok(HttpResponse::Ok().json(emps));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn getbyempid(req:HttpRequest, empid:web::Path<String>) -> Result<HttpResponse,MyError>{
    if let Some(header) = req.headers().get(http::header::CONTENT_TYPE){
        if let Ok(_s) = header.to_str(){
        let emp = hr::getbyempid(empid.into_inner()).await?;
        return Ok(HttpResponse::Ok().json(emp));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn update(req:HttpRequest,emp:web::Json<Hr>) -> Result<HttpResponse,MyError>{
    if let Some(header) = req.headers().get(http::header::CONTENT_TYPE){
        if let Ok(_s) = header.to_str(){
            let em = Hr::new(emp.emp_id.to_string(),emp.dep_id.to_string(),emp.salary);
            let message = hr::update(em).await?;
            return Ok(HttpResponse::Ok().json(message));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn insert(req:HttpRequest,emp:web::Json<Hr>) -> Result<HttpResponse,MyError>{
    if let Some(header) = req.headers().get(http::header::CONTENT_TYPE){
        if let Ok(_s) = header.to_str(){
            let em = Hr::new(emp.emp_id.to_string(),emp.dep_id.to_string(),emp.salary);
            let message = hr::insert(em).await?;
            return Ok(HttpResponse::Ok().json(message));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn delete(req: HttpRequest, empid:web::Path<String>) -> Result<HttpResponse,MyError>{
    if let Some(header) = req.headers().get(http::header::CONTENT_TYPE){
        if let Ok(_s) = header.to_str(){
            let message = hr::delete(empid.into_inner()).await?;
            return Ok(HttpResponse::Ok().json(message));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}