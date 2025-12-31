-- ALTER TABLE t_generated_games RENAME TO t_generated_games_old;

-- CREATE TABLE t_generated_games (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     concurso INTEGER NOT NULL,
--     data TEXT NOT NULL,
--     bola_1 INTEGER,
--     bola_2 INTEGER,
--     bola_3 INTEGER,
--     bola_4 INTEGER,
--     bola_5 INTEGER,
--     bola_6 INTEGER,
--     inserted_at TEXT DEFAULT (datetime('now'))
-- );

-- INSERT INTO t_generated_games (
--     id, concurso, data,
--     bola_1, bola_2, bola_3,
--     bola_4, bola_5, bola_6,
--     inserted_at
-- )
-- SELECT
--     id,
--     0 AS concurso,                 -- valor padrão, ajuste se necessário
--     created_at AS data,
--     n1, n2, n3, n4, n5, n6,
--     created_at AS inserted_at
-- FROM t_generated_games_old;

-- DROP TABLE t_generated_games_old;

-- CREATE INDEX idx_generated_games_bola_1 ON t_generated_games (bola_1);
-- CREATE INDEX idx_generated_games_bola_2 ON t_generated_games (bola_2);
-- CREATE INDEX idx_generated_games_bola_3 ON t_generated_games (bola_3);
-- CREATE INDEX idx_generated_games_bola_4 ON t_generated_games (bola_4);
-- CREATE INDEX idx_generated_games_bola_5 ON t_generated_games (bola_5);
-- CREATE INDEX idx_generated_games_bola_6 ON t_generated_games (bola_6);
