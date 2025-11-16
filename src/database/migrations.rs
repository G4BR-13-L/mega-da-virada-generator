use crate::shared::sha3;
use anyhow::{Context, Result};
use chrono::Utc;
use rand::seq::IteratorRandom;
use rusqlite::{Connection, OptionalExtension, params};
use std::io::{BufReader, Read};
use std::path::Path;
use std::{fs, num};

pub fn check_migration_table_exists(conn: &Connection) -> Result<bool> {
    let exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='t_migration'",
            [],
            |row| row.get(0),
        )
        .context("Erro ao checar tabela t_migration")?;
    Ok(exists > 0)
}

pub fn create_migration_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS t_migration (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_name TEXT NOT NULL UNIQUE,
            checksum_sha3 TEXT NOT NULL,
            executed_at TEXT DEFAULT (datetime('now'))
        )",
        [],
    )?;
    Ok(())
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    let migrations_dir = Path::new("./migrations");
    if !migrations_dir.exists() {
        println!("Diretório ./migrations não encontrado — pulando migrations.");
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(migrations_dir)?
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sql") {
            continue;
        }
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        println!("Migration encontrada: {}", file_name);
        let file_sha3 = sha3::sha3_256_of_file(&path)?;

        let maybe_db_sha3: Option<String> = conn
            .query_row(
                "SELECT checksum_sha3 FROM t_migration WHERE file_name = ?1",
                params![file_name],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(db_sha3) = maybe_db_sha3 {
            if db_sha3 != file_sha3 {
                panic!(
                    "O checksum SHA3 do arquivo {} não corresponde ao registrado no banco.",
                    file_name
                );
            } else {
                println!("Já executado e verificado: {}", file_name);
                continue;
            }
        }

        let sql_content = fs::read_to_string(&path)?;
        conn.execute_batch(&sql_content)?;

        conn.execute(
            "INSERT INTO t_migration (file_name, checksum_sha3) VALUES (?1, ?2)",
            params![file_name, file_sha3],
        )?;

        println!("Executada migration: {}", file_name);
    }

    Ok(())
}
