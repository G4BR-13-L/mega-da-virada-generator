CREATE TABLE IF NOT EXISTS t_historico_mega_sena (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            concurso INTEGER NOT NULL,
            data TEXT NOT NULL,
            bola_1 INTEGER, bola_2 INTEGER, bola_3 INTEGER,
            bola_4 INTEGER, bola_5 INTEGER, bola_6 INTEGER,
            inserted_at TEXT DEFAULT (datetime('now'))
)
