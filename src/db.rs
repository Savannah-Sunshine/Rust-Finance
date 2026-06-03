use crate::models::{Transaction, Category};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct TransactionRepo {
    data: Arc<RwLock<Vec<Transaction>>>,
}

impl TransactionRepo {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

pub trait TransactionRepository {
    fn get_all(&self) -> Vec<Transaction>;
    fn get_by_id(&self, id: u32) -> Option<Transaction>;
    fn create(&self, transaction: Transaction);
    fn get_by_category(&self, category: Category) -> Vec<Transaction>;
}

impl TransactionRepository for TransactionRepo {
    fn get_all(&self) -> Vec<Transaction> {
        self.data.read().unwrap().clone()
    }

    fn get_by_category(&self, category: Category) -> Vec<Transaction> {
        self.data.read().unwrap().iter().filter(|t| t.category == category).cloned().collect()
    }
    
    fn get_by_id(&self, id: u32) -> Option<Transaction> {
        self.data
            .read()
            .unwrap()
            .iter()
            .find(|t| t.id == id)
            .cloned()
    }

    fn create(&self, transaction: Transaction) {
        self.data.write().unwrap().push(transaction);
    }
}