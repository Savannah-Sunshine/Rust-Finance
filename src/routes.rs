use axum::{extract::{Path, State}, Json};
use crate::models::{Transaction, TransactionRequest, Category, TransactionRepository, AppState};

pub async fn get_transactions(State(state): State<AppState>,) -> Json<Vec<Transaction>> {
    Json(state.repo.get_all())
}

pub async fn create_transaction(State(state): State<AppState>, Json(payload ): Json<TransactionRequest>,) -> Json<Transaction> {
    let transaction = Transaction::new(payload);
    state.repo.create(transaction.clone());
    Json(transaction)
}

pub async fn get_category(Path(category): Path <Category>, State(state): State<AppState>) -> Json<Vec<Transaction>> {
    Json(state.repo.get_by_category(category))
}