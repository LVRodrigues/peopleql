use crate::model::{prelude::*, *};
use async_graphql::{ComplexObject, Context, Object};
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

#[ComplexObject]
impl cards::Model {
    async fn cards(&self, ctx: &Context<'_>) -> Result<Vec<questions::Model>, DbErr> {
        let connection = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(Questions).all(connection).await
    }
}
