use futures::{TryStreamExt};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use mongodb::{bson::doc,Collection, Database};

const COLLECTION_NAME: &str = "pets";

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}
#[derive(Serialize, Deserialize)]
pub struct Pet {
    pub name: String,
}

// This function will get all pets from the database
// It takes a Database as an argument
pub async fn get_all_pets_from_db(database:Database) -> Result<Vec<Pet>> {
    let collection: Collection<Pet> = database.collection(COLLECTION_NAME);
    let cursor = collection.find(doc!{}).await?;
    let pets: Vec<Pet> = cursor.try_collect().await?;
    Ok(pets)
}

// This function will insert a pet into the database
// It takes a Database and a Pet as arguments
pub async fn insert_pet_into_db(database: Database, pet: Pet) -> Result<()> {
    let collection: Collection<Pet> = database.collection(COLLECTION_NAME);
    collection.insert_one(pet).await?;
    Ok(())
}