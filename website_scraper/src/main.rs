use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

mod models;
use crate::models::ShipmentUpdate;

mod read_web;
use crate::read_web::collect_from_website;


const NAME_DB: &str = "ukb-website";


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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

   collect_from_website().await?;

   /*// Load the MongoDB connection string from an environment variable:
   let client_uri = env::var("MONGODB_URI")
       .expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

   let data_point = ShipmentUpdate {
      id: None,
      updated: chrono::Utc::now(),
      order_range_l: 100,
      order_range_r: 120
   };


   insert_update(&client, &data_point).await?;*/

   Ok(())
}