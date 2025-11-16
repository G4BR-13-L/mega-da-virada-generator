use anyhow::{Context, Result};
use rusqlite::{Connection, params};

pub fn ingest_csv_mega_sena_to_sqlite(conn: &mut Connection, csv_path: &str) -> Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(csv_path)
        .with_context(|| format!("Falha ao abrir CSV {}", csv_path))?;

    let tx = conn.transaction()?;
    let mut inserted = 0usize;

    for result in rdr.records() {
        let record = result?;
        if record.len() < 8 {
            eprintln!("Linha ignorada (colunas insuficientes): {:?}", record);
            continue;
        }

        let strip_quotes = |s: &str| -> String {
            s.trim()
                .trim_matches('\'')
                .trim_matches('"')
                .trim()
                .to_string()
        };

        let concurso: i64 = strip_quotes(&record[0]).parse()?;
        let data = strip_quotes(&record[1]);
        let bola_1: i64 = strip_quotes(&record[2]).parse()?;
        let bola_2: i64 = strip_quotes(&record[3]).parse()?;
        let bola_3: i64 = strip_quotes(&record[4]).parse()?;
        let bola_4: i64 = strip_quotes(&record[5]).parse()?;
        let bola_5: i64 = strip_quotes(&record[6]).parse()?;
        let bola_6: i64 = strip_quotes(&record[7]).parse()?;

        tx.execute(
            "INSERT INTO t_historico_mega_sena
                (concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6
            ],
        )?;
        inserted += 1;
    }

    tx.commit()?;
    println!("Ingestão concluída. {} linhas inseridas.", inserted);
    Ok(())
}

pub fn ingest_csv_lotofacil_to_sqlite(conn: &mut Connection, csv_path: &str) -> Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(csv_path)
        .with_context(|| format!("Falha ao abrir CSV {}", csv_path))?;

    let tx = conn.transaction()?;
    let mut inserted = 0usize;

    for result in rdr.records() {
        let record = result?;
        if record.len() < 17 {
            eprintln!("Linha ignorada (colunas insuficientes): {:?}", record);
            continue;
        }

        let strip_quotes = |s: &str| -> String {
            s.trim()
                .trim_matches('\'')
                .trim_matches('"')
                .trim()
                .to_string()
        };

        let concurso: i64 = strip_quotes(&record[0]).parse()?;
        let data = strip_quotes(&record[1]);
        let bola_1: i64 = strip_quotes(&record[2]).parse()?;
        let bola_2: i64 = strip_quotes(&record[3]).parse()?;
        let bola_3: i64 = strip_quotes(&record[4]).parse()?;
        let bola_4: i64 = strip_quotes(&record[5]).parse()?;
        let bola_5: i64 = strip_quotes(&record[6]).parse()?;
        let bola_6: i64 = strip_quotes(&record[7]).parse()?;
        let bola_7: i64 = strip_quotes(&record[8]).parse()?;
        let bola_8: i64 = strip_quotes(&record[9]).parse()?;
        let bola_9: i64 = strip_quotes(&record[10]).parse()?;
        let bola_10: i64 = strip_quotes(&record[11]).parse()?;
        let bola_11: i64 = strip_quotes(&record[12]).parse()?;
        let bola_12: i64 = strip_quotes(&record[13]).parse()?;
        let bola_13: i64 = strip_quotes(&record[14]).parse()?;
        let bola_14: i64 = strip_quotes(&record[15]).parse()?;
        let bola_15: i64 = strip_quotes(&record[16]).parse()?;

        tx.execute(
            "INSERT INTO t_historico_lotofacil
                (
                concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6
                , bola_7, bola_8, bola_9, bola_10, bola_11, bola_12, bola_13, bola_14, bola_15
                )
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6, bola_7, bola_8,
                bola_9, bola_10, bola_11, bola_12, bola_13, bola_14, bola_15,
            ],
        )?;
        inserted += 1;
    }

    tx.commit()?;
    println!("Ingestão concluída. {} linhas inseridas.", inserted);
    Ok(())
}
