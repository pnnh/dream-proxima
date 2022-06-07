use async_graphql::{Context, Object, Result};

use crate::graphql::types::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let user = User {
            id: "a".to_string(),
            display_name: "b".to_string(),
        };
        let result = vec![user];

        Ok(result)
    }

    async fn get_user(&self, ctx: &Context<'_>, id: String) -> Result<Option<User>> {
        tracing::debug!("create_post {:?}", id);

        let result = User {
            id: "a".to_string(),
            display_name: "b".to_string(),
        };

        Ok(Option::from(result))
    }
}
