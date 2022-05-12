use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleView {
    pub pk: String,
    pub title: String,
}
