use std::collections::HashSet;

use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::domain::mega_sena::MegaSena;

pub fn listar_historico(conn: &Connection) -> Result<Vec<MegaSena>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, concurso, data,
                bola_1, bola_2, bola_3,
                bola_4, bola_5, bola_6,
                inserted_at
         FROM t_mega_sena
         ORDER BY concurso ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        // Coleta todas as bolas em um vetor
        let bolas: [Option<i64>; 6] = [
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
        ];

        // Cria e preenche o HashSet
        let mut set = HashSet::new();
        for b in bolas {
            if let Some(v) = b {
                set.insert(v);
            }
        }

        Ok(MegaSena {
            id: row.get(0)?,
            concurso: row.get(1)?,
            data: row.get(2)?,
            bola_1: bolas[0],
            bola_2: bolas[1],
            bola_3: bolas[2],
            bola_4: bolas[3],
            bola_5: bolas[4],
            bola_6: bolas[5],
            inserted_at: row.get(9)?,
            generated_by_rust: false,
            set,
        })
    })?;

    // Coleta tudo para um Vec<HistoricoMegaSena>
    let historico: Vec<MegaSena> = rows
        .filter_map(|r| r.ok()) // descarta linhas com erro
        .collect();

    Ok(historico)
}

pub(crate) fn save(conn: &mut Connection, mega_sena: &MegaSena) -> Result<(), anyhow::Error> {
    let tx = conn.transaction()?;

    let concurso = Uuid::new_v4().to_string();
    let data = chrono::Local::now().format("%d/%m/%Y").to_string();
    let data_typesafe = chrono::Local::now().format("%Y-%m-%d").to_string();
    let bola_1 = mega_sena.bola_1;
    let bola_2 = mega_sena.bola_2;
    let bola_3 = mega_sena.bola_3;
    let bola_4 = mega_sena.bola_4;
    let bola_5 = mega_sena.bola_5;
    let bola_6 = mega_sena.bola_6;
    let generated_by_rust = mega_sena.generated_by_rust;

    tx.execute(
        "INSERT INTO t_mega_sena
            (concurso, data, data_typesafe, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6, generated_by_rust)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            concurso, data, data_typesafe, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6, generated_by_rust
        ],
    )?;

    tx.commit()?;
    Ok(())
}
