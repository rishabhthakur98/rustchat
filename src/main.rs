use serde::{Deserialize, Serialize};
use mongodb::{Client, bson::doc};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://root:example@localhost:27017").await?;
    let coll = client.database("testdb").collection::<User>("users");

    coll.insert_one(User { name: "Bob".into(), age: 25 }, None).await?;

    let user = coll.find_one(doc! { "name": "Bob" }, None).await?;
    println!("{:?}", user);

    Ok(())
}