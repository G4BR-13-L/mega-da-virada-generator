use rusqlite::{Connection, OptionalExtension, params};

use crate::core::mega_sena::MegaSena;

/// Verifica se o jogo já existe no histórico
pub fn game_already_exists(
    conn: &Connection,
    megasena: &MegaSena,
) -> Result<bool, rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT EXISTS(
                SELECT 1 FROM t_historico_mega_sena
                WHERE bola_1=?1 AND bola_2=?2 AND bola_3=?3
                  AND bola_4=?4 AND bola_5=?5 AND bola_6=?6
            )",
        params![
            megasena.jogo[0],
            megasena.jogo[1],
            megasena.jogo[2],
            megasena.jogo[3],
            megasena.jogo[4],
            megasena.jogo[5],
        ],
        |row| row.get(0),
    )?;

    Ok(exists)
}

/// Função principal que o usuário vai chamar.
/// Verifica se algum trio já saiu no passado.
pub fn has_repeated_trio(conn: &Connection, megasena: &MegaSena) -> Result<bool, rusqlite::Error> {
    // gerar trios corretamente
    let trios = generate_trios(&megasena.jogo);

    // delegar para a função que faz a consulta real
    has_repeated_trio_by_sets(conn, &trios)
}

/// Gera todas as combinações de trios (C(6,3)=20) para um jogo de 6 números.
/// O jogo deve ter exatamente 6 números.
pub fn generate_trios(game: &[i64]) -> Vec<[i64; 3]> {
    assert!(game.len() == 6, "O jogo deve conter exatamente 6 números!");

    let mut trios = Vec::new();

    for i in 0..4 {
        for j in (i + 1)..5 {
            for k in (j + 1)..6 {
                trios.push([game[i], game[j], game[k]]);
            }
        }
    }

    trios
}

/// Verifica se algum trio gerado aparece junto em um mesmo concurso do passado.
/// Retorna true se encontrar ao menos 1 ocorrência.
pub fn has_repeated_trio_by_sets(
    conn: &Connection,
    trios: &Vec<[i64; 3]>,
) -> Result<bool, rusqlite::Error> {
    let sql = "
        SELECT concurso
        FROM t_historico_mega_sena
        WHERE bola_1 IN (?1,?2,?3)
           OR bola_2 IN (?1,?2,?3)
           OR bola_3 IN (?1,?2,?3)
           OR bola_4 IN (?1,?2,?3)
           OR bola_5 IN (?1,?2,?3)
           OR bola_6 IN (?1,?2,?3)
        GROUP BY concurso
        HAVING COUNT(*) >= 3
        LIMIT 1;
    ";

    for trio in trios {
        let exists: Option<i32> = conn
            .query_row(sql, params![trio[0], trio[1], trio[2]], |row| row.get(0))
            .optional()?;

        if exists.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}
