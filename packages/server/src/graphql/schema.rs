use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{Html, IntoResponse};
use axum::Extension;
use std::sync::Arc;

use crate::graphql::{mutation::MutationRoot, query::QueryRoot};
use crate::handlers::State;
use crate::models::claims::Claims;

pub async fn graphql_mutation_handler<'a>(
    claims: Option<Claims>,
    Extension(state): Extension<Arc<State<'static>>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(state)
    .data(claims)
    .finish();
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_mutation_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/graphql/mutation",
    )))
}
