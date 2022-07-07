use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};

pub struct Article {
    pub title: String,
}

#[Object]
impl Article {
    async fn title(&self) -> &str {
        self.title.as_str()
    }
}
