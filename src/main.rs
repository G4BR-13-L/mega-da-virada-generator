use anyhow::Result;
use log::info;
use rusqlite::Connection;

use crate::{
    application::{
        generator::RandomMegaSenaGenerator, service::MegaSenaService, validator::MegaSenaValidator,
    },
    domain::heuristics::SomaRange,
    infraestructure::{mega_sena_bootstrap, migrations::run_migrations, repository},
};

const QTD_TOLERAVEL: u8 = 4;
const QTD_JOGOS_DESEJADOS: u8 = 10;
const DB_PATH: &str = "mega_sena.db";

pub mod application;
pub mod domain;
pub mod infraestructure;
pub mod shared;

fn main() -> Result<()> {
    let mut conn = Connection::open(DB_PATH)?;

    let _ = match run_migrations() {
        Ok(_) => info!("Success running migrations"),
        Err(_) => panic!("Failed to run migrations"),
    };

    let _ = match mega_sena_bootstrap::bootstrap_mega_sena_data_from_csv() {
        Ok(_) => info!("Success extrecting historic data of mega sena."),
        Err(_) => panic!("Failed to extract historic data of me."),
    };

    let historico = repository::listar_historico(&conn)?;

    let soma_range = SomaRange::new(&historico);

    let service = MegaSenaService {
        generator: RandomMegaSenaGenerator,
        validator: MegaSenaValidator {
            soma_range,
            tolerancia: QTD_TOLERAVEL,
        },
    };

    let jogos = service.gerar_jogos(QTD_JOGOS_DESEJADOS, &historico);

    for jogo in jogos {
        repository::save(&mut conn, &jogo)?;
        println!("{}", jogo);
    }

    Ok(())
}
