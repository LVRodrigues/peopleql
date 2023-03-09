use crate::model::{prelude::*, *};
use async_graphql::{Context, Object};
use sea_orm::*;

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }

    async fn cards(&self, ctx: &Context<'_>) -> Result<Vec<cards::Model>, DbErr> {
        let connection = ctx.data::<DatabaseConnection>().unwrap();
        Cards::find().all(connection).await
    }
}

