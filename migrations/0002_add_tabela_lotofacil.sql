CREATE TABLE IF NOT EXISTS t_historico_lotofacil (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            concurso INTEGER NOT NULL,
            data TEXT NOT NULL,
            bola_1 INTEGER, bola_2 INTEGER, bola_3 INTEGER,
            bola_4 INTEGER, bola_5 INTEGER, bola_6 INTEGER,
            bola_7 INTEGER, bola_8 INTEGER, bola_9 INTEGER,
            bola_10 INTEGER, bola_11 INTEGER, bola_12 INTEGER,
            bola_13 INTEGER, bola_14 INTEGER, bola_15 INTEGER,
            inserted_at TEXT DEFAULT (datetime('now'))
)
