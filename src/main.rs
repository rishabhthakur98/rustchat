
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection, Database
};
use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;
mod lib;
use lib::mongodbcollections::User;
use lib::handlers::{loginuser, registeruser};

#[tokio::main]
async fn main() {
    match Client::with_uri_str("mongodb://root:example@localhost:27017").await{
        Ok(client) => {
    let db = client.database("userdb");

    let cors = CorsLayer::permissive();  
    let app = Router::new()
        .route("/register", post(registeruser))
        .route("/login", post(loginuser))
        .with_state(db)
        .layer(cors);
        match tokio::net::TcpListener::bind("0.0.0.0:8000").await{
        Ok(listener) => {
            match axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            
            {
                Ok(_server) => {

                },
                Err(e) => {
                        eprintln!("Server error: {}", e);
                }
            }
        },
        Err(e) => {
                eprintln!("Failed to bind: {}", e);
        }
    }
        },
        Err(e)=> {
            eprintln!("Database connection error: {}", e);
        }
    }
}




async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");

    println!("Signal received. Shutting down gracefully...");
}

