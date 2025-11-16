use anyhow::{Context, Result};
use chrono::Utc;
use rand::seq::IteratorRandom;
use rusqlite::{Connection, OptionalExtension, params};
use std::io::{BufReader, Read};
use std::path::Path;
use std::{fs, num};

pub mod core;
pub mod database;
pub mod engine;
pub mod shared;

use core::mega_sena;
use database::{csv, migrations};
use engine::game_generator;
use shared::sha3;

const IS_DADOS_INGERIDOS: bool = true;

fn main() -> Result<()> {
    let db_path = "mega_sena.db";
    let csv_mega_sena_path = "mega_sena.csv";
    let csv_lotofacil_path = "loto_facil.csv";

    let mut conn = Connection::open(db_path)?;
    println!("Conectado ao SQLite em {}", db_path);

    if !migrations::check_migration_table_exists(&conn)? {
        println!("Tabela t_migration não existe. Criando...");
        migrations::create_migration_table(&conn)?;
    }

    database::migrations::run_migrations(&conn)?;

    if !IS_DADOS_INGERIDOS {
        if Path::new(csv_mega_sena_path).exists() {
            println!("Iniciando ingestão do CSV '{}'", csv_mega_sena_path);
            database::csv::ingest_csv_mega_sena_to_sqlite(&mut conn, csv_mega_sena_path)?;
        } else {
            println!(
                "Arquivo CSV '{}' não encontrado — pulando ingestão.",
                csv_mega_sena_path
            );
        }

        if Path::new(csv_lotofacil_path).exists() {
            println!("Iniciando ingestão do CSV '{}'", csv_lotofacil_path);
            database::csv::ingest_csv_lotofacil_to_sqlite(&mut conn, csv_lotofacil_path)?;
        } else {
            println!(
                "Arquivo CSV '{}' não encontrado — pulando ingestão.",
                csv_lotofacil_path
            );
        }
    }

    for i in 0..1000000 {
        let generated_mega_sena: mega_sena::MegaSena =
            engine::game_generator::generate_mega_sena(&conn)?;

        let game_already_existis: bool =
            match engine::analyser::game_already_exists(&conn, &generated_mega_sena) {
                Ok(true) => true,
                Ok(false) => false,
                Err(e) => {
                    println!("Erro ao verificar: {}", e);
                    false
                }
            };

        let repeated_trio: bool =
            match engine::analyser::has_repeated_trio(&conn, &generated_mega_sena) {
                Ok(true) => true,
                Ok(false) => false,
                Err(e) => {
                    println!("Erro ao verificar: {}", e);
                    false
                }
            };

        if i % 1000 == 0 {
            println!("...");
            println!("Iteração [ {} ]", i);
            println!("...");
        }
        if game_already_existis || repeated_trio {
            println!("Iteração [ {} ]", i);
            println!("---------------------------------------------");
            println!("Numeros gerados: {}", generated_mega_sena);

            println!("O jogo existe na historia?: {}", game_already_existis);

            println!("O jogo possui um trio repetido?: {}", repeated_trio);
            println!("---------------------------------------------");
        }
    }

    Ok(())
}
