use async_graphql::{EmptySubscription, Schema};
use std::sync::Arc;

use crate::graphql::{mutation::MutationRoot, query::QueryRoot};
use crate::handlers::State;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the PrismaClient to the context
pub async fn build_schema(state: Arc<State<'static>>) -> AppSchema {
    // let db = db::new_client()
    //     .await
    //     .expect("Failed to create Prisma client");

    // For more information about schema data, see: https://async-graphql.github.io/async-graphql/en/context.html#schema-data
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(state)
    .finish()
}
