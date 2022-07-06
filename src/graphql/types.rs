use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};

#[derive(SimpleObject, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub display_name: String,
}

#[ComplexObject]
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

#[derive(SimpleObject, Debug)]
pub struct Article {
    pub title: String,
}
