use async_graphql::{Context, InputObject, Object, Result};

use crate::graphql::types::User;

#[derive(InputObject, Debug)]
pub struct CreateUserInput {
    pub display_name: String,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, _ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        tracing::debug!("create_user {:?}", input);

        let result = User {
            id: "a".to_string(),
            display_name: "b".to_string(),
        };

        Ok(result)
    }
}
