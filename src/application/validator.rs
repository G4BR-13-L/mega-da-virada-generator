use crate::domain::{heuristics::SomaRange, mega_sena::MegaSena, rules};

pub struct MegaSenaValidator {
    pub soma_range: SomaRange,
    pub tolerancia: u8,
}

impl MegaSenaValidator {
    pub fn is_jogavel(&self, jogo: &MegaSena, historico: &[MegaSena]) -> bool {
        if !self.soma_range.contains(jogo.soma()) {
            return false;
        }

        if rules::excede_ocorrencias(jogo, historico, self.tolerancia) {
            return false;
        }

        true
    }
}
