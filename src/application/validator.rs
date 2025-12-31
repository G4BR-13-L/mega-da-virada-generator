use crate::domain::{heuristics::{FaixaEstatistica, SomaRange}, mega_sena::MegaSena, rules};

pub struct MegaSenaValidator {
    pub soma_range: SomaRange,
    pub tolerancia: u8,
    pub faixa_estatistica: FaixaEstatistica,
}

impl MegaSenaValidator {
    pub fn is_jogavel(&self, jogo: &MegaSena, historico: &[MegaSena]) -> bool {
        if !self.soma_range.contains(jogo.soma()) {
            return false;
        }

        if rules::excede_ocorrencias(jogo, historico, self.tolerancia) {
            return false;
        }

        if !rules::respeita_faixa_estatistica(jogo, &self.faixa_estatistica) {
            return false;
        }

        true
    }
}
