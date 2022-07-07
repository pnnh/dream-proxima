pub mod article;

use crate::graphql::query::article::IndexQuery;
pub use article::ArticleQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct QueryRoot(ArticleQuery, IndexQuery);
