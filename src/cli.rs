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

    #[command(about="Pushes a `deposit` transaction to the ledger")]
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
}
