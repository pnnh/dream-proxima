use std::sync::Arc;

use crate::handlers::State;
use crate::models::article::ArticleModel;
use crate::models::error::{AppError, OtherError};
use crate::models::index::IndexModel;

pub struct IndexService {
    state: Arc<State>,
}

impl IndexService {
    pub fn new(state: Arc<State>) -> IndexService {
        IndexService { state }
    }

    pub async fn query(&self, offset: i64, limit: i64) -> Result<Vec<IndexModel>, AppError> {
        let conn = self
            .state
            .pool
            .get()
            .await
            .map_err(|err| OtherError::BB8Postgres(err))?;

        let query_result = conn
            .query(
                "select articles.pk, articles.title, articles.body, 
articles.description, articles.update_time, articles.creator, articles.keywords,
accounts.nickname, articles_views.views
from articles
    left join accounts on articles.creator = accounts.pk
	left join articles_views on articles.pk = articles_views.pk
where articles.status = 1
order by update_time desc offset $1 limit $2;",
                &[&offset, &limit],
            )
            .await
            .map_err(|err| AppError::Postgresql(err))?;

        let mut models: Vec<IndexModel> = Vec::new();

        for row in query_result {
            let pk: &str = row.get("pk");
            let title: &str = row.get("title");
            let body: serde_json::Value = row.get("body");
            let description: Option<&str> = row.get("description");
            let update_time: chrono::NaiveDateTime = row.get("update_time");
            let creator: String = row.get("creator");
            let keywords: Option<&str> = row.get("keywords");
            let creator_nickname: Option<&str> = row.get("nickname");
            let views: Option<i64> = row.get("views");

            let model = IndexModel {
                pk: pk.to_string(),
                title: title.to_string(),
                body,
                description: description.unwrap_or("").to_string(),
                update_time_formatted: update_time.format("%Y年%m月%d日 %H:%M").to_string(),
                creator: creator.to_string(),
                creator_nickname: creator_nickname.unwrap_or("").to_string(),
                views: views.unwrap_or(0),
                keywords: keywords.unwrap_or("").to_string(),
            };
            models.push(model);
        }
        Ok(models)
    }
}
