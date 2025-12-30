use rusqlite::{Connection, OptionalExtension};
use std::path::Path;

use crate::infraestructure::csv::ingest_csv_mega_sena_to_sqlite;

const CSV_MEGA_SENA_PATH: &str = "mega_sena.csv";
const DB_PATH: &str = "mega_sena.db";

pub fn bootstrap_mega_sena_data_from_csv() -> anyhow::Result<()> {
    let mut conn = Connection::open(DB_PATH)?;
    println!("Conectado ao SQLite em {}", DB_PATH);

    if !mega_sena_table_has_data(&conn)? {
        if Path::new(CSV_MEGA_SENA_PATH).exists() {
            println!("Iniciando ingestão do CSV '{}'", CSV_MEGA_SENA_PATH);
            ingest_csv_mega_sena_to_sqlite(&mut conn, CSV_MEGA_SENA_PATH)?;
        } else {
            println!(
                "Arquivo CSV '{}' não encontrado — pulando ingestão.",
                CSV_MEGA_SENA_PATH
            );
        }
    }
    return Ok(());
}

pub fn mega_sena_table_has_data(conn: &Connection) -> Result<bool, rusqlite::Error> {
    let sql = "
        SELECT id
        FROM t_mega_sena
        LIMIT 1;
    ";

    let exists: Option<i32> = conn.query_row(sql, [], |row| row.get(0)).optional()?;

    if exists.is_some() {
        return Ok(true);
    }

    Ok(false)
}
