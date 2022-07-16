
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexModel {
    pub pk: String,
    pub title: String,
    pub body: serde_json::Value,
    pub creator: String,
    pub keywords: String,
    pub description: String,
    pub update_time_formatted: String,
    pub creator_nickname: String,
    pub views: i64,
}
