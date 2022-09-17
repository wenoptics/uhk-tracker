use std::env;
use std::error::Error;
use mongodb::{
    Client,
    options::{ClientOptions, ResolverConfig},
};

use crate::models::ShipmentUpdate;

const NAME_DB: &str = "ukb-website";

///
/// Insert a ShipmentUpdate to the database for persistence
///
async fn insert_update(client: &Client, su: &ShipmentUpdate) -> Result<(), Box<dyn Error>> {
    let coll_updates = client.database(NAME_DB).collection("shipment_updates");

    // Convert to a Bson instance:
    let ser_dp = bson::to_bson(&su)?;
    let doc = ser_dp.as_document().unwrap();

    // Insert into the collection and extract the inserted_id value:
    let insert_result = coll_updates.insert_one(doc.to_owned(), None).await?;
    let id_dp = insert_result
        .inserted_id
        .as_object_id()
        .expect("Retrieved _id should have been of type ObjectId");
    println!("The inserted document ID: {:?}", id_dp);

    Ok(())
}

pub async fn save_to_db(data_point: &ShipmentUpdate) -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    /*// Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
       println!("- {}", name);
    }*/

    insert_update(&client, &data_point).await?;

    Ok(())
}
