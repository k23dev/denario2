use actix_web::{Responder, HttpResponse, get,post,patch,delete};

use crate::core::feature_core::FeatureCore;

#[get("/credits")]
pub async fn find_all() -> impl Responder {
    let fcore=FeatureCore::load();

    let mut credits: Vec<String> = Vec::new();
    credits.push("Actix Web".to_string());
    credits.push("Diesel".to_string());
    credits.push("PostgreSQL".to_string());
    credits.push("Rust".to_string());
    
    HttpResponse::Ok().json({
        credits
    })
}

#[get("/credits/{id}")]
pub async fn find_one(id: i32) -> impl Responder {
    let fcore=FeatureCore::load();
    let credits = find_all();
    return credits[id as usize].clone();
    
    HttpResponse::Ok().json({
        credits
    })
}

#[post("/credits")]
pub async fn create(credit: String) -> impl Responder {
    let fcore=FeatureCore::load();
    // let mut credits = find_all();
    // credits.push(credit);
    // return credits[credits.len() - 1].clone();

    HttpResponse::Ok().json({
        credits
    })
}

#[patch("/credits/{id}")]
pub async fn update(id: i32, credit: String) -> impl Responder {
    let fcore=FeatureCore::load();
    // let mut credits = find_all();
    // credits[id as usize] = credit;
    // return credits[id as usize].clone();

    HttpResponse::Ok().json({
        credits
    })
}

#[delete("/credits/{id}")]
pub async fn delete(id: i32) -> impl Responder {
    let fcore=FeatureCore::load();
    // let mut credits = find_all();
    // credits.remove(id as usize);
    // return credits[id as usize].clone();

    HttpResponse::Ok().json({
        credits
    })
}