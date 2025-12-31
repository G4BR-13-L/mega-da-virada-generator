use super::mega_sena::MegaSena;
use crate::domain::heuristics::FaixaEstatistica;


pub fn excede_ocorrencias(jogo: &MegaSena, historico: &[MegaSena], tolerancia: u8) -> bool {
    historico.iter().any(|h| jogo.intersecao(h) >= tolerancia)
}

pub fn respeita_faixa_estatistica(
    jogo: &MegaSena,
    faixa: &FaixaEstatistica,
) -> bool {
    let count = jogo
        .set
        .iter()
        .filter(|&&n| n >= faixa.min && n <= faixa.max)
        .count() as i64;

    let (min, max) = faixa.intervalo_valido();
    count >= min && count <= max
}
