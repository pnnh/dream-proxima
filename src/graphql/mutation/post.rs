use async_graphql::{Context, InputObject, Object, Result};

use crate::graphql::types::Post;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject, Debug)]
pub struct CreatePostInput {
    pub content: String,
    // this really would be grabbed from session or something but just for demo
    pub user_id: String,
}

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_post(&self, _ctx: &Context<'_>, input: CreatePostInput) -> Result<Post> {
        tracing::debug!("create_post {:?}", input);

        let result = Post {
            id: "a".to_string(),
            content: "b".to_string(),
            user_id: "c".to_string(),
        };

        Ok(result)
    }
}
