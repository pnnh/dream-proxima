use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexArticleView {
    pub pk: String,
    pub title: String,
    pub body: String,
    pub creator: String,
    pub keywords: String,
    pub description: String,
    pub update_time: chrono::NaiveDateTime,
    pub creator_nickname: String,
    pub views: i32,
}
