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

#[cfg(test)]
mod tests{
    const DATABASE: &str = "petdb";
    const COLLECTION_NAME: &str = "pets";
    use super::*;
    use mongodb::{Client, Collection};
    use crate::services::pets::{Pet};
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn get_all_pets_ok() {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
        let client = Client::with_uri_str(&mongo_uri).await;
        assert!(client.is_ok());
        let db = client.unwrap().database(DATABASE);
        //insert a document for testing, clear old data
        let collection: Collection<Pet> = db.collection(COLLECTION_NAME);
        let res = collection.drop().await;
        assert!(res.is_ok());
        let result = collection.insert_one(Pet{
            name: "Rover".to_string(),
        }).await;
        assert!(result.is_ok());
        //pass connection  to the app state for reuse
        let app_state = services::pets::AppState { database: db };
        let app = test::init_service(App::new()
        .app_data(web::Data::new(app_state))
        .service(get_all_pets)).await;
        let req = test::TestRequest::get().uri("/api/v1/pets").to_request();
        let resp:Vec<Pet>= test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.len(),1);
        assert_eq!(resp[0].name,"Rover");
    }

    #[actix_web::test]
    async fn create_pet_ok() {
        let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
        let client = Client::with_uri_str(&mongo_uri).await;
        assert!(client.is_ok());
        let db = client.unwrap().database(DATABASE);
        //insert a document for testing, clear old data
        let collection: Collection<Pet> = db.collection(COLLECTION_NAME);
        let res = collection.drop().await;
        assert!(res.is_ok());
        //pass connection  to the app state for reuse
        let app_state = services::pets::AppState { database: db };
        let app = test::init_service(App::new()
        .app_data(web::Data::new(app_state))
        .service(create_pet)).await;
        let req = test::TestRequest::post().uri("/api/v1/pets")
        .insert_header(ContentType::json())
        .set_json(&Pet{
            name: "Rover".to_string(),
        }).to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}",resp);
        assert!(resp.status().is_success());
    }
}