use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use native_model::{native_model, Model};
use native_db::{native_db, Models, ToKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionKind {
    Deposit,
    Widthdraw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id=1, version=1)]
#[native_db]
pub struct Transaction {
    #[primary_key]
    pub id: i64,
    pub time: DateTime<Utc>,
    pub kind: TransactionKind,
    pub amount: u16, // u16 cuz you shouldn't be managaing amounts larger than 65,535 on this piece of software (not that secure)
    pub tags: Box<[String]>,
}

pub fn models() -> Models {
    let mut models = Models::new();
    models.define::<Transaction>().unwrap();

    models
}

/// Converts transactions to RON
#[inline]
pub fn to_ron(transactions: &[Transaction]) -> String {
    ron::to_string(transactions).unwrap()
}

/// Converts RON to transactions
#[inline]
pub fn from_ron(ron: &str) -> Box<[Transaction]> {
    ron::from_str(ron).expect("provided RON is invalid (for transactions)")
}
