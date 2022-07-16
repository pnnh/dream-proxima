use std::sync::Arc;

use crate::handlers::State;
use crate::models::article::ArticleModel;
use crate::models::error::{AppError, HttpError, OtherError};
use crate::models::index::IndexModel;

pub struct IndexService {
    state: Arc<State>,
}

impl IndexService {
    pub fn new(state: Arc<State>) -> IndexService {
        IndexService { state }
    }

    pub async fn query(&self, offset: i64, limit: i64) -> Result<Vec<IndexModel>, HttpError> {
        let conn = self
            .state
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
where articles.status = 1
order by update_time desc offset $1 limit $2;",
                &[&offset, &limit],
            )
            .await
            .map_err(|err| AppError::Postgresql(err))?;

        let mut models: Vec<IndexModel> = Vec::new();

        for row in query_result {
            let pk: &str = row.get(0);
            let title: &str = row.get("title");
            let body: serde_json::Value = row.get(2);
            let description: &str = row.get("description");
            let update_time: chrono::NaiveDateTime = row.get(4);
            let creator: String = row.get(5);
            let keywords: String = row.get(6);
            let creator_nickname: &str = row.get(7);
            let views: Option<i64> = row.get(8);

            let model = IndexModel {
                pk: pk.to_string(),
                title: title.to_string(),
                body,
                description: description.to_string(),
                update_time_formatted: update_time.format("%Y年%m月%d日 %H:%M").to_string(),
                creator: creator.to_string(),
                creator_nickname: creator_nickname.to_string(),
                views: views.unwrap_or(0),
                keywords,
            };
            models.push(model);
        }
        Ok(models)
    }
}
