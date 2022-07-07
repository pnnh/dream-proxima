use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};

pub struct User {
    pub id: String,
    pub display_name: String,
}

#[Object]
impl User {
    pub async fn posts(&self, _ctx: &Context<'_>) -> Result<Vec<Post>> {
        let post = Post {
            id: "a".to_string(),
            content: "b".to_string(),
            user_id: "c".to_string(),
        };
        let result = vec![post];

        Ok(result)
    }
}

pub struct Post {
    pub id: String,
    pub content: String,
    pub user_id: String,
}

#[Object]
impl Post {
    async fn id(&self) -> &str {
        self.id.as_str()
    }
}

pub struct Article {
    pub title: String,
}

#[Object]
impl Article {
    async fn title(&self) -> &str {
        self.title.as_str()
    }
}
