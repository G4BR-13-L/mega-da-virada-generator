use super::mega_sena::MegaSena;

pub fn excede_ocorrencias(jogo: &MegaSena, historico: &[MegaSena], tolerancia: u8) -> bool {
    historico.iter().any(|h| jogo.intersecao(h) >= tolerancia)
}
