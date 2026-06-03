use serde::{Deserialize, Serialize};
use chrono::{Local, Utc};
use uuid::Uuid;
use std::sync::{Arc, RwLock};

/** ! DB stuff */
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

/** ! Data stuff */
#[derive(Clone)]
pub struct AppState {
    pub repo: TransactionRepo,
}

// Categories
// income, rent, utilities, dog, food, exercise, education, transportation, subscriptions, clothing, plants, misc
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    Income,
    Rent,
    Utilities,
    Dog,
    Food,
    Exercise,
    Education,
    Transportation,
    Subscriptions,
    Clothing,
    Plants,
    Misc
}
// Place, Amount, Date, Note, Card
pub type Place = String;
pub type Amount = f64;
pub type Date = String;
pub type Note = String;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Card {
    Visa,
    CapitalOne,
    Debit,
    Other
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransactionRequest {
    pub category: Category,
    pub place: Place,
    pub amount: Amount,
    pub note: Note,
    pub card: Card,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub category: Category,
    pub place: Place,
    pub amount: Amount,
    pub date: Date,
    pub note: Note,
    pub card: Card,
}

impl Transaction {
    pub fn new(TransactionRequest { category, place, amount, note, card }: TransactionRequest) -> Self {
        Transaction {
            id: Uuid::new_v4().as_u128() as u32,
            category,
            place,
            amount,
            date: Utc::now().format("%d-%m%Y").to_string(),
            note,
            card
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {} at {} on {} using {:?} - {}", self.category as u8, self.amount, self.place, self.date, self.card, self.note)
    }
}