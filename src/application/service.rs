use super::{generator::MegaSenaGenerator, validator::MegaSenaValidator};
use crate::domain::mega_sena::MegaSena;

pub struct MegaSenaService<G: MegaSenaGenerator> {
    pub(crate) generator: G,
    pub(crate) validator: MegaSenaValidator,
}

impl<G: MegaSenaGenerator> MegaSenaService<G> {
    pub fn gerar_jogos(&self, quantidade: u8, historico: &[MegaSena]) -> Vec<MegaSena> {
        let mut jogos = Vec::new();

        while jogos.len() < quantidade as usize {
            let jogo = self.generator.generate();

            if self.validator.is_jogavel(&jogo, historico) {
                jogos.push(jogo);
            }
        }

        jogos
    }
}
