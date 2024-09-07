use std::{collections::HashMap, path::Path};
use log::{info, warn};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use native_model::{native_model, Model};
use native_db::{native_db, transaction::RwTransaction, Builder, Database, Models, ToKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionKind {
    Deposit,
    Withdraw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id=1, version=1)]
#[native_db]
pub struct Transaction {
    #[primary_key]
    pub id: i64,
    /// The type of currency to be recorded (eg `USD`)
    #[secondary_key]
    pub currency: String,

    pub time: DateTime<Utc>,
    pub kind: TransactionKind,
    pub amount: f32,
    pub message: Option<String>,
    pub tags: Box<[String]>,
}

pub fn models() -> Models {
    let mut models = Models::new();
    models.define::<Transaction>().unwrap();

    models
}

pub fn open_or_create<'a>(path: impl AsRef<Path>, models: &'a Models) -> Database<'a> {
    // check if it exists or not
    if path.as_ref().is_file() {
        info!("found and opened transaction database");
        Builder::new()
            .open(models, path)
            .unwrap()
    } else {
        warn!("transaction database not found; creating a new one");
        Builder::new()
            .create(models, path)
            .unwrap()
    }
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

pub fn get_sorted_transactions(rw: &RwTransaction) -> Vec<Transaction> {
    info!("sorting database transactions");
    
    // get the transactions in the database and sort them
    let mut transactions = rw.scan()
        .primary::<Transaction>()
        .unwrap()
        .all()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    transactions.sort_unstable_by_key(|taction| taction.id);

    transactions
}

pub fn get_balance(rw: &RwTransaction) -> HashMap<String, f64> {
    let mut balances = HashMap::new();
    let transactions = get_sorted_transactions(rw);

    // iterate through the transactions and update the balances
    for taction in transactions {
        let balance = balances.get(&taction.currency).unwrap_or(&0.0);
        balances.insert(
            taction.currency,
            match taction.kind {
                TransactionKind::Deposit  => balance + taction.amount as f64,
                TransactionKind::Withdraw => balance - taction.amount as f64,
            },
        );
    }

    balances
}
