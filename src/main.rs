use std::io::{stdin, stdout, BufRead, Write};

use anyhow::Context;
use rsqlite::db;

fn main() -> Result<(), anyhow::Error> {
    let database = db::Db::from_file(std::env::args().nth(1).context("db file is required")?)?;
    cli(database)
}

fn cli(mut db: db::Db) -> Result<(), anyhow::Error> {
    print_flushed("rsqlite> ")?;

    let mut line_buffer = String::new();

    while stdin().lock().read_line(&mut line_buffer).is_ok() {
        match line_buffer.trim() {
            ".exit" => break,
            ".tables" => display_tables(&mut db)?,
            _ => println!("Unrecognized command '{}'", line_buffer.trim()),
        }

        print_flushed("\nrsqlite> ")?;
        line_buffer.clear();
    }

    Ok(())
}

fn display_tables(db: &mut db::Db) -> Result<(), anyhow::Error> {
    let mut scanner = db.scanner(1);

    while let Some(mut record) = scanner.next_record()? {
        let type_value = record
            .field(0)
            .context("missing type field")
            .context("invalid type field")?;

        if type_value.as_str() == Some("table") {
            let name_value = record
                .field(1)
                .context("missing name field")
                .context("invalid name field")?;

            print!("{} ", name_value.as_str().unwrap());
        }
    }

    Ok(())
}

fn print_flushed(s: &str) -> Result<(), anyhow::Error> {
    print!("{s}");
    stdout().flush().context("failed to flush stdout")
}
