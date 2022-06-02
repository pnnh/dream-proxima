use async_graphql::{Context, Object, Result};

use crate::graphql::types::Article;

#[derive(Default)]
pub struct ArticleQuery;

#[Object]
impl ArticleQuery {
    async fn articles(&self, ctx: &Context<'_>) -> Result<Vec<Article>> {
        let article = Article {
            id: "a".to_string(),
            display_name: "b".to_string(),
            title: "title".to_string(),
        };
        let result = vec![article];

        Ok(result)
    }
}
