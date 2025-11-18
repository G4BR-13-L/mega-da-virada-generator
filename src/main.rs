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

use crate::core::historico_mega_sena::HistoricoMegaSena;
use crate::engine::analyser;

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

    let historico_mega_sela_list = match analyser::listar_historico_mega_sena(&conn) {
        Ok(r) => r,
        Err(_) => panic!("❌ Erro ao carregar histórico da Mega-Sena"),
    };

    let mut jogos_jogaveis_desejados: u8 = 10;
    let mut jogos_gerados: Vec<HistoricoMegaSena> = Vec::with_capacity(jogos_jogaveis_desejados as usize);
    while jogos_jogaveis_desejados > 0 {
        let generated_mega_sena: HistoricoMegaSena = engine::game_generator::generate_mega_sena()?;

        let mut ocorrencias_encontradas = false;
        const QTD_TOLERAVEL: u8 = 4;
        const PRINT_NAO_JOGAVEL: bool = false;

        for h in &historico_mega_sela_list {
            let mut contagem_ocorrencias: u8 = 0;

            for numero in generated_mega_sena.set.clone() {
                if h.set.contains(&numero) {
                    contagem_ocorrencias += 1;
                }
            }

            if PRINT_NAO_JOGAVEL && contagem_ocorrencias >= QTD_TOLERAVEL {
                ocorrencias_encontradas = true;

                println!(
                    "\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                 🚫 JOGO BLOQUEADO\n\
                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                 O jogo {} NÃO deve ser jogado.\n\
                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
                    generated_mega_sena
                );

                println!("Motivo:");
                println!(
                    "• Pelo menos {QTD_TOLERAVEL} números coincidem com um concurso anterior."
                );
                println!("• Concurso Nº: {}", h.concurso);
                println!("• Bolas do concurso: {}", h);
                println!("• Data: {}", h.data);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

                break;
            }
        }

        if !ocorrencias_encontradas {
            jogos_jogaveis_desejados -= 1;
            jogos_gerados.push(generated_mega_sena.clone());
            // println!(
            //     "\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            //  ✅ JOGO PERMITIDO\n\
            //  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
            //  O jogo {} pode ser jogado.\n\
            //  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
            //     generated_mega_sena
            // );
        }
    }

    let mut soma_minima = 346;
    let mut soma_maxima = 0;
    for j in historico_mega_sela_list {
        let soma = j.bola_1.unwrap_or(0) +
           j.bola_2.unwrap_or(0) +
          j.bola_3.unwrap_or(0) +
         j.bola_4.unwrap_or(0) +
        j.bola_5.unwrap_or(0) +
       j.bola_6.unwrap_or(0);
        if soma < soma_minima {
            soma_minima = soma;
        }

        if soma > soma_maxima {
            soma_maxima = soma;
        }
    }


    println!("\n\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Soma minima {}", soma_minima);
    println!("Soma maxima {}", soma_maxima);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut jogos_com_soma_valida: Vec<HistoricoMegaSena> = Vec::with_capacity(jogos_gerados.len());
    for jogo in &jogos_gerados {
        let mut soma_jogo = 0;
        for n in &jogo.set {
            soma_jogo+=n;
        }
        if soma_jogo > soma_minima && soma_jogo < soma_maxima {
            jogos_com_soma_valida.push(jogo.clone());
        }
    }

    for jogo in jogos_com_soma_valida {
        println!(
            "\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
         ✅ JOGO PERMITIDO\n\
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
         O jogo {} pode ser jogado.\n\
         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
         jogo
        );
    }

    Ok(())
}
