use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema, EmptyMutation,
};
use async_graphql_rocket::*;
use rocket::{response::content, *};
use schema::*;
use sea_orm::*;
// use sea_orm::{Database, DatabaseConnection, DbErr};

mod model;
mod schema;

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

const DATABASE: &str = "postgres://guess-user:guess-pass@localhost:5432/guess-who";

async fn connect() -> Result<DatabaseConnection, DbErr> {
    let connection = Database::connect(DATABASE).await?;
    Ok(connection)
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

#[rocket::get("/graphql")]
fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, Luciano!"
}

#[launch]
async fn rocket() -> _ {
    let connection = match connect().await {
        Ok(connection) => connection,
        Err(err) => panic!("{}", err),
    };
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(connection)
        .finish();
    rocket::build()
        .manage(schema)
        .mount("/", routes![index, graphql_request, graphql_playground])
}
