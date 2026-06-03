use serde::{Deserialize, Serialize};
use chrono::{Local, Utc};
use uuid::Uuid;

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