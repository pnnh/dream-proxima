use crate::graphql::types::Article;
use crate::handlers::State;
use crate::models::error::AuthError;
use async_graphql::{Context, InputObject, Object, Result};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(InputObject, Debug)]
pub struct CreateArticleInput {
    title: String,
    body: String,
    publish: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct ArticleBody {
    children: String,
}

#[derive(Debug, Serialize)]
pub struct CreateBody {
    pk: String,
}

#[Object]
impl CreateBody {
    async fn pk(&self) -> &str {
        self.pk.as_str()
    }
}

#[derive(Default)]
pub struct ArticleMutation;

#[Object]
impl ArticleMutation {
    pub async fn create_article(
        &self,
        ctx: &Context<'_>,
        input: CreateArticleInput,
    ) -> Result<CreateBody> {
        tracing::debug!("create_post {:?}", input);
        let state = ctx.data::<Arc<State>>().unwrap();

        let conn = state
            .pool
            .get()
            .await
            .map_err(|err| AuthError::InvalidToken)?;

        let pk = nanoid!(12);
        let article_body = ArticleBody {
            children: "children".to_string(),
        };
        let publish = if input.publish { 1 } else { 0 };
        conn
            .execute(
                "insert into articles(pk, title, body, create_time, update_time, creator, keywords, description, status)
    values($1, $2, $3,'2022-07-05 21:54:53','2022-07-05 21:54:57','x','y','z', $4);",
                &[&pk,
                    &input.title,
                    &postgres_types::Json::<ArticleBody>(article_body),
                    &publish,
                ],
            )
            .await
            .map_err(|err| AuthError::Postgresql(err))?;

        let result = CreateBody { pk: pk };
        Ok(result)
    }
}
