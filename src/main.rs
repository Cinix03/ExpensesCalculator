use firebase_rs::Firebase;

mod model;
mod persistence;
mod routes;
use persistence::ExpenseRepository;
use persistence::UserRepository;

use crate::model::User;

#[tokio::main]
async fn main() {
    let app = routes::create_router();

    dotenv::dotenv().ok();
    let firebase_url = std::env::var("FIREBASE_URL").expect("FIREBASE_URL not set");

    let firebase = Firebase::new(&firebase_url).unwrap();

    let user_repo = UserRepository::new(&firebase);
    let expense_reop = ExpenseRepository::new(&firebase);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
