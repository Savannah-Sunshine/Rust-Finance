use axum::{extract::{Path, State}, routing::get, Json, Router,};
mod db;
mod models;
use models::{Transaction, TransactionRequest, Category, Card};
use db::TransactionRepo;

use crate::db::TransactionRepository;


#[derive(Clone)]
pub struct AppState {
    pub repo: TransactionRepo,
}


#[tokio::main]
async fn main() {
    let state = AppState {
        repo: TransactionRepo::new(),
    };

    // fill with dummy data
    let transaction_request = TransactionRequest {
        category: Category::Food,
        place: "McDonald's".to_string(),
        amount: 5.99,
        note: "Lunch".to_string(),
        card: Card::Visa,
    };
    state.repo.create(Transaction::new(transaction_request));


    // build our server and routes
    let app = Router::new()
        .route("/transactions", get(get_transactions).post(create_transaction))
        .route("/transactions/{category}", get(get_category))
        .with_state(state);
    //axum handles all error routes

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_transactions(State(state): State<AppState>,) -> Json<Vec<Transaction>> {
    Json(state.repo.get_all())
}

async fn create_transaction(State(state): State<AppState>, Json(payload ): Json<TransactionRequest>,) -> Json<Transaction> {
    let transaction = Transaction::new(payload);
    state.repo.create(transaction.clone());
    Json(transaction)
}

async fn get_category(Path(category): Path <Category>, State(state): State<AppState>) -> Json<Vec<Transaction>> {
    Json(state.repo.get_by_category(category))
}