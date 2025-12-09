use mongodb::{Collection, Database};
use super::mongodbcollections::User;
use axum::{
    Json, body::Body, extract::{ Path, State}, response:: Response
};
use serde_json::{json, Value};


pub async fn registeruser(
    State(db): State<Database>
)
-> Response<Body> {
        let coll: Collection<User> = db.collection("users");
        println!("inside register user {:?}", db);
        let mut status_code = 200;
        let mut body = String::from("");
    match coll.insert_one(User { name: "Bob".into(), age: 25 }, None).await{
        Ok(val) => {
            status_code = 200;
            body = String::from("Ok done");
        },
        Err(e) => {
            status_code = 500;
            body = String::from("Error {:?}");
        }
    }

    let response = Response::builder()
    .status(status_code)
    .header("Content-Type", "application/json")
    .body(Body::from(body))
    .unwrap();
    return response
}




pub async fn loginuser(){
        // let coll: Collection<User> = db.collection("users");
}



// #[tokio::main]
// async fn main(){
 

//     coll.insert_one(User { name: "Bob".into(), age: 25 }, None).await;

//     let user = coll.find_one(doc! { "name": "Bob" }, None).await;
//     println!("{:?}", user);





// pub async fn getvalue(
//     State(data): State<Arc<Mutex<HashMap<String, String>>>>,
//     Path(key): Path<String>
// ) -> Response<Body> {
//     let map = data.lock().await;
//     println!("entire map: {:?}", *map);
//     let value = map.get(&key).unwrap();
//     let res_body:String = format!(r#"{{"{}":"{:?}"}}"#, key, value);
//     let body = Body::from(res_body);
//     let response = Response::builder()
//     .status(200)
//     .header("Content-Type", "application/json")
//     .body(body)
//     .unwrap();
//     return response
// }



// pub async fn insertkeyvalue(
//     State(data): State<Arc<Mutex<HashMap<String, String>>>>,
//     body: String
// )-> Response<Body> {

// let json_data: Value = serde_json::from_str(&body).unwrap();

// let mut hash_map = data.lock().await;

// for (key, value) in json_data.as_object().unwrap() {
//     hash_map.insert(key.clone(), value.as_str().unwrap_or(&value.to_string()).to_string());
//     }
//     let res_body:String = format!(r#"{{"status":"done"}}"#);
//     let res_body = Body::from(res_body);

//     Response::builder()
//     .status(200)
//     .header("Content-Type", "application/json")
//     .body(res_body)
//     .unwrap()
//     }
