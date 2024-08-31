use std::fs;

use chrono::Utc;
use clap::Parser;
use log::info;
use yarl::{cli::{Cli, Command}, database::{self, to_ron, Transaction, TransactionKind}, log::{panic_hook, Logger}};

pub static LOGGER: Logger = Logger;

fn main() {
    // set logger & panic handling
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info));
    std::panic::set_hook(Box::new(panic_hook));

    // parse cli
    let cli = Cli::parse();

    // open database
    let models = database::models();
    let database = database::open_or_create(&models);
    let rw = database.rw_transaction().unwrap();

    // handle cli
    match cli.command {
        Command::Test => info!("hello, world!"),
        Command::Deposit { time, currency, amount, message, tags } => {
            let time = time.unwrap_or_else(|| Utc::now());
            rw.insert(Transaction {
                id: time.timestamp_millis(),
                currency,
                time,
                kind: TransactionKind::Deposit,
                amount,
                message,
                tags: tags.into_boxed_slice(),
            }).expect("failed to insert transaction into ledger");
        },
        Command::Withdraw { time, currency, amount, message, tags } => {
            let time = time.unwrap_or_else(|| Utc::now());
            rw.insert(Transaction {
                id: time.timestamp_millis(),
                currency,
                time,
                kind: TransactionKind::Withdraw,
                amount,
                message,
                tags: tags.into_boxed_slice(),
            }).expect("failed to insert transaction into ledger");
        },
        Command::Balance => {
            info!("getting & displaying balances");
            let balances = database::get_balance(&rw);
            for (key, val) in balances.into_iter() {
                info!("\x1b[33m[{key} balance]:\x1b[0m {}", val);
            }
        },
        Command::Export { path, tags, currency } => {
            let transactions = database::get_sorted_transactions(&rw)
                .into_iter()
                .filter(|taction| tags.iter().all(|tag| taction.tags.contains(tag)))
                .filter(|taction| currency.as_ref().map(|c| taction.currency == *c).unwrap_or(true))
                .collect::<Vec<_>>();

            let ron = to_ron(&transactions);
            fs::write(&path, ron).expect(&format!("failed to write exported RON to {path}"));
        },
    }

    rw.commit().unwrap();
    info!("successfully completed all actions");
}
