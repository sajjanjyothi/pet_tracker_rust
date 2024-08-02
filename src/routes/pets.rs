#![allow(unused_imports)]
use log::error;
use actix_web::{cookie::time::error, get, post, web, App, FromRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use crate::services;

#[get("/api/v1/pets")]
pub async fn get_all_pets(state: web::Data<services::pets::AppState>) -> impl Responder {
    let pets = services::pets::get_all_pets_from_db(state.database.clone()).await;
    match pets {
        Ok(pets) => HttpResponse::Ok().json(pets),
        Err(e) => {
            error!("An error occurred {}",e);
            HttpResponse::InternalServerError().body("An error occurred")
        }     
    }
}

#[post("/api/v1/pets")]
pub async fn create_pet(state: web::Data<services::pets::AppState>,pet: web::Json<services::pets::Pet>) -> impl Responder {
   let res = services::pets::insert_pet_into_db(state.database.clone(),pet.into_inner()).await;
   match res {
       Ok(_) => HttpResponse::Created().body("Pet created successfully"),
       Err(e) => {
           error!("An error occurred {}",e);
           HttpResponse::InternalServerError().body("An error occurred")
       }
    }
}