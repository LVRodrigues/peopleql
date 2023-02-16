use sea_orm::{DatabaseConnection, DbErr, Database};
use rocket::*;

mod model;

const DATABASE: &str = "postgres://guess-user:guess-pass@localhost:5432/guess-who";

async fn connect() -> Result<DatabaseConnection, DbErr> {
    let connection = Database::connect(DATABASE).await?;
    Ok(connection)
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
    rocket::build()
        .manage(connection)
        .mount("/", routes![index])
}