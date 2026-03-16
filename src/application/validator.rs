use crate::domain::{heuristics::SomaRange, mega_sena::MegaSena, rules};

pub struct MegaSenaValidator {
    pub soma_range: SomaRange,
    pub tolerancia: u8,
}

impl MegaSenaValidator {
    pub fn is_jogavel(&self, jogo: &MegaSena, historico: &[MegaSena]) -> bool {
        let nums = &jogo.set;

        if !self.soma_range.contains(jogo.soma()) {
            return false;
        }

        if !Self::paridade_balanceada(nums) {
            return false;
        }

        if !Self::distribuicao_dezenas(nums) {
            return false;
        }

        if !Self::limite_consecutivos(nums) {
            return false;
        }

        if !Self::range_minimo(nums) {
            return false;
        }

        if !Self::balanceamento_baixo_alto(nums) {
            return false;
        }

        if rules::excede_ocorrencias(jogo, historico, self.tolerancia) {
            return false;
        }

        true
    }

    fn paridade_balanceada(nums: &std::collections::HashSet<i64>) -> bool {
        let pares = nums.iter().filter(|n| **n % 2 == 0).count();
        pares >= 2 && pares <= 4
    }

    fn distribuicao_dezenas(nums: &std::collections::HashSet<i64>) -> bool {
        let mut buckets = [false; 6];

        for n in nums {
            let idx = ((*n as usize) - 1) / 10;
            buckets[idx] = true;
        }

        buckets.iter().filter(|b| **b).count() >= 4
    }

    fn limite_consecutivos(nums: &std::collections::HashSet<i64>) -> bool {
        let mut v: Vec<i64> = nums.iter().cloned().collect();
        v.sort_unstable();

        let mut seq = 1;

        for i in 1..v.len() {
            if v[i] == v[i - 1] + 1 {
                seq += 1;
                if seq >= 3 {
                    return false;
                }
            } else {
                seq = 1;
            }
        }

        true
    }

    fn range_minimo(nums: &std::collections::HashSet<i64>) -> bool {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        max - min > 25
    }

    fn balanceamento_baixo_alto(nums: &std::collections::HashSet<i64>) -> bool {
        let baixos = nums.iter().filter(|n| **n <= 30).count();
        baixos >= 2 && baixos <= 4
    }
}
