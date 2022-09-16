use bson::oid::ObjectId;
use chrono::Utc;
use serde::{Deserialize, Serialize};

// You use `serde` to create structs which can serialize & deserialize between BSON:
#[derive(Serialize, Deserialize, Debug)]
pub struct ShipmentUpdate {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated: chrono::DateTime<Utc>,
    pub raw_post: String,

    // Order shipment range
    pub order_range_l: Option<u32>,
    pub order_range_r: Option<u32>,
}
