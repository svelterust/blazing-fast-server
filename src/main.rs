use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use server::{error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create server
    let app = Router::new().route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok(axum::serve(listener, app).await?)
}

// User route
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(Json(data): Json<CreateUser>) -> Result<Json<User>> {
    // Create user if username is not admin
    if data.username != "admin" {
        Ok(Json(User {
            id: 1337,
            username: data.username,
        }))
    } else {
        Err(error!("Creating admin user is illegal"))?
    }
}
