use home::home_dir;
use log::{info, warn};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use native_model::{native_model, Model};
use native_db::{native_db, Builder, Database, Models, ToKey};

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
    /// The type of currency to be recorded (eg `USD`)
    #[secondary_key]
    pub currency: String,

    pub time: DateTime<Utc>,
    pub kind: TransactionKind,
    pub amount: u16, // u16 cuz you shouldn't be managaing amounts larger than 65,535 on this piece of software (not that secure)
    pub message: Option<String>,
    pub tags: Box<[String]>,
}

pub fn models() -> Models {
    let mut models = Models::new();
    models.define::<Transaction>().unwrap();

    models
}

pub fn open_or_create<'a>(models: &'a Models) -> Database<'a> {
    let path = home_dir()
        .expect("unable to get home directory")
        .join(".yarl.redb");

    // check if it exists or not
    if path.is_file() {
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
