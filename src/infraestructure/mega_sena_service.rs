use anyhow::Result;
use rand::seq::IteratorRandom;
use rusqlite::{Connection, OptionalExtension, params};
use std::collections::HashSet;
use uuid::Uuid;

use crate::domain::mega_sena::MegaSena;



/// Consulta um jogo gerado pelo id e imprime.
pub fn query_generated_game(conn: &Connection, id: i64) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, n1, n2, n3, n4, n5, n6, created_at FROM t_generated_games WHERE id = ?1",
    )?;
    let row_opt = stmt
        .query_row(params![id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, String>(7)?,
            ))
        })
        .optional()?;

    if let Some((id, n1, n2, n3, n4, n5, n6, created_at)) = row_opt {
        println!(
            "Jogo id={} gerado em {}: [{}, {}, {}, {}, {}, {}]",
            id, created_at, n1, n2, n3, n4, n5, n6
        );
    } else {
        println!("Nenhum jogo encontrado com id {}", id);
    }

    Ok(())
}

/// Verifica se o jogo gerado já existe no histórico.
pub fn query_generated_game_in_history(conn: &Connection, game: &MegaSena) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6
         FROM t_mega_sena
         WHERE bola_1 = ?1 AND bola_2 = ?2 AND bola_3 = ?3
           AND bola_4 = ?4 AND bola_5 = ?5 AND bola_6 = ?6",
    )?;

    let row_opt = stmt
        .query_row(
            params![
                game.bola_1,
                game.bola_2,
                game.bola_3,
                game.bola_4,
                game.bola_5,
                game.bola_6
            ],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,    // concurso
                    row.get::<_, String>(1)?, // data
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                ))
            },
        )
        .optional()?;

    if let Some((concurso, data, b1, b2, b3, b4, b5, b6)) = row_opt {
        println!(
            "Jogo já existente no histórico (concurso {} - {}): [{}, {}, {}, {}, {}, {}]",
            concurso, data, b1, b2, b3, b4, b5, b6
        );
    } else {
        println!("Jogo inédito! Nenhum registro encontrado no histórico.");
    }

    Ok(())
}




