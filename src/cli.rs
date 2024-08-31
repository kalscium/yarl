use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about="A mere test command")]
    Test,

    #[command(about="Gets the current balance of your account")]
    Balance,

    #[command(about="Exports the ledger (most recent transaction first) in RON format")]
    Export {
        #[arg(index=1, help="The path to export the ledger to")]
        path: String,
        #[arg(short, long, num_args=.., help="Filters the exported transactions to have to contain the specified tags")]
        tags: Vec<String>,
        #[arg(short, long, help="Filters the exported transactions by a currency")]
        currency: Option<String>,
    },

    #[command(about="Lists all the tags used in the ledger (most recent first)")]
    Tags,

    #[command(about="Inserts a `deposit` transaction into the ledger")]
    Deposit {
        #[arg(short='i', long, help="Sets the time of which the transaction had occured (defaults to now)")]
        time: Option<DateTime<Utc>>,
        #[arg(short, long, default_value_t={"USD".to_string()}, help="The kind of currency the transaction involves")]
        currency: String,
        #[arg(index=1, help="The amount you've deposited")]
        amount: u16,
        #[arg(short, long, help="A message that describes the purpose of this transaction")]
        message: Option<String>,
        #[arg(short, long, num_args=.., help="The tags to give the transaction")]
        tags: Vec<String>,
    },

    #[command(about="Inserts a `withdraw` transaction into the ledger")]
    Withdraw {
        #[arg(short='i', long, help="Sets the time of which the transaction had occured (defaults to now)")]
        time: Option<DateTime<Utc>>,
        #[arg(short, long, default_value_t={"USD".to_string()}, help="The kind of currency the transaction involves")]
        currency: String,
        #[arg(index=1, help="The amount you've withdrawn")]
        amount: u16,
        #[arg(short, long, help="A message that describes the purpose of this transaction")]
        message: Option<String>,
        #[arg(short, long, num_args=.., help="The tags to give the transaction")]
        tags: Vec<String>,
    },
}
