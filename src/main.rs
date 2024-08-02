use actix_web::{ web, App, HttpServer};
use anyhow::Result;
use log::info;
use mongodb::Client;
mod routes;
mod services;


#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting http server");
    //creating mongo db connection
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&mongo_uri).await.expect("Failed to connect to MongoDB");
    let db = client.database("mydatabase");
    //pass connection  to the app state for reuse
    let app_state = services::pets::AppState { database: db };
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(app_state.clone()))
        .service(routes::pets::get_all_pets)
        .service(routes::pets::create_pet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}