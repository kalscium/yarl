use chrono::Utc;
use clap::Parser;
use native_db::Builder;
use yarl::{cli::Cli, database::{self, from_ron, to_ron, Transaction, TransactionKind}};

fn main() {
    let _cli = Cli::parse();
    
    let models = database::models();
    let database = Builder::new().create_in_memory(&models).unwrap();
    let rw = database.rw_transaction().unwrap();

    let time = Utc::now();
    let transaction = Transaction {
        id: time.timestamp_millis(),
        time,
        kind: TransactionKind::Deposit,
        amount: 12423,
        tags: Box::new([]),
    };

    let ron = to_ron(&[transaction]);
    println!("{ron}");
    let transaction = from_ron(&ron)[0].clone();

    rw.insert(transaction).unwrap();
    rw.commit().unwrap();
}
