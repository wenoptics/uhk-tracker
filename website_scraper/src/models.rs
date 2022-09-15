use bson::oid::ObjectId;
use chrono::Utc;
use serde::{Deserialize, Serialize};

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
pub struct ShipmentUpdate {
   #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub(crate) id: Option<ObjectId>,
   #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
   pub(crate) updated: chrono::DateTime<Utc>,
   pub(crate) order_range_l: u32,
   pub(crate) order_range_r: u32,
}
