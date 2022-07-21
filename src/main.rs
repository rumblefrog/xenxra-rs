use std::env;

use csv::{ReaderBuilder, Trim};

mod account;
mod ledger;
mod raw_csv;
mod transaction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ledger = ledger::Ledger::new();

    let mut res = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(env::args().nth(1).unwrap())?;

    for r in res.deserialize() {
        let tx: raw_csv::Transaction = r?;

        ledger.process(tx);
    }

    let mut write = csv::Writer::from_writer(std::io::stdout());

    for acc in ledger.snapshot() {
        write.serialize(acc)?;
    }

    write.flush()?;

    Ok(())
}
