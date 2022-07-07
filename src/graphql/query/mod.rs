pub mod article;

pub use article::ArticleQuery;

#[derive(async_graphql::MergedObject, Default)]
pub struct QueryRoot(ArticleQuery);
