use async_graphql::{Context, Object, Result};

use crate::graphql::types::Post;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn get_posts(&self, _ctx: &Context<'_>) -> Result<Vec<Post>> {
        let post = Post {
            id: "a".to_string(),
            content: "b".to_string(),
            user_id: "c".to_string(),
        };
        let result = vec![post];

        Ok(result)
    }
}
