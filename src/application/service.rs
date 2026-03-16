use super::{generator::MegaSenaGenerator, validator::MegaSenaValidator};
use crate::domain::mega_sena::MegaSena;
use rayon::prelude::*;

pub struct MegaSenaService<G: MegaSenaGenerator + Sync + Send> {
    pub(crate) generator: G,
    pub(crate) validator: MegaSenaValidator,
}

impl<G: MegaSenaGenerator + Sync + Send> MegaSenaService<G> {
    pub fn gerar_jogos(&self, quantidade: u8, historico: &[MegaSena]) -> Vec<MegaSena> {
        let mut jogos = Vec::new();

        while jogos.len() < quantidade as usize {
            let mut novos: Vec<MegaSena> = (0..10_000)
                .into_par_iter()
                .map(|_| self.generator.generate())
                .filter(|j| self.validator.is_jogavel(j, historico))
                .collect();

            jogos.append(&mut novos);
        }

        jogos.truncate(quantidade as usize);
        jogos
    }
}
