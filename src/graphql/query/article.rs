use async_graphql::{Context, Object, Result};
use std::sync::Arc;

use crate::graphql::types::Article;
use crate::handlers::State;

#[derive(Default)]
pub struct IndexQuery;

#[Object]
impl IndexQuery {
    async fn articles(&self, ctx: &Context<'_>, offset: i32, limit: i32) -> Result<Vec<Article>> {
        let state = ctx.data::<Arc<State>>().unwrap();

        let offset_value: i64 = if offset < 0 { 0 } else { offset as i64 };
        let limit_value: i64 = if limit < 4 || limit > 64 {
            8
        } else {
            limit as i64
        };

        let conn = state
            .pool
            .get()
            .await
            .expect("graphql articles获取pool出错");

        let query_result = conn
            .query(
                "select articles.pk, articles.title, articles.body, 
articles.description, articles.update_time, articles.creator, articles.keywords,
accounts.nickname, articles_views.views
from articles
    left join accounts on articles.creator = accounts.pk
	left join articles_views on articles.pk = articles_views.pk
order by update_time desc offset $1 limit $2;",
                &[&offset_value, &limit_value],
            )
            .await
            .expect("graphql articles执行查询出错");

        let mut result: Vec<Article> = Vec::new();

        for row in query_result {
            let title: &str = row.get("title");

            let model = Article {
                title: title.to_string(),
            };
            result.push(model);
        }

        Ok(result)
    }

    async fn count(&self, ctx: &Context<'_>) -> Result<i32> {
        let state = ctx.data::<Arc<State>>().unwrap();

        let conn = state.pool.get().await.expect("graphql count获取pool出错");

        let query_result = conn
            .query("select count(*) from articles;", &[])
            .await
            .expect("graphql count执行查询出错");

        for row in query_result {
            let count: i64 = row.get(0);
            return Ok(count as i32);
        }

        Err(async_graphql::Error::new("分页错误"))
    }
}
