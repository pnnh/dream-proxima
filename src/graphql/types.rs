use async_graphql::{ComplexObject, Context, Result, SimpleObject};

#[derive(SimpleObject, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub display_name: String,
}

#[ComplexObject]
impl User {
    pub async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let post = Post {
            id: "a".to_string(),
            content: "b".to_string(),
            user_id: "c".to_string(),
        };
        let result = vec![post];

        Ok(result)
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub user_id: String,
}

#[ComplexObject]
impl Post {
    pub async fn user(&self, ctx: &Context<'_>) -> Result<Option<Box<User>>> {
        let user = User {
            id: "a".to_string(),
            display_name: "b".to_string(),
        };

        Ok(Option::from(Box::new(user)))
    }
}

#[derive(SimpleObject, Debug)]
pub struct Article {
    pub title: String,
}
