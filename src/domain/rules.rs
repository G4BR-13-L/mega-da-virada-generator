use super::mega_sena::MegaSena;
use rayon::prelude::*;

pub fn excede_ocorrencias(jogo: &MegaSena, historico: &[MegaSena], tolerancia: u8) -> bool {
    historico
        .par_iter()
        .any(|h| jogo.intersecao(h) >= tolerancia)
}
