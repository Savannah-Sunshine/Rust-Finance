use axum::{routing::get, Router,};
mod models;
use models::{Transaction, TransactionRequest, Category, Card, TransactionRepo,TransactionRepository, AppState};
mod routes;


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
        .route("/transactions", get(routes::get_transactions).post(routes::create_transaction))
        .route("/transactions/{category}", get(routes::get_category))
        .with_state(state);
    //axum handles all error routes

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}