use std::collections::HashSet;

use rand::seq::IteratorRandom;

use crate::domain::mega_sena::MegaSena;

pub trait MegaSenaGenerator {
    fn generate(&self) -> MegaSena;
}

pub struct RandomMegaSenaGenerator;

impl MegaSenaGenerator for RandomMegaSenaGenerator {
    fn generate(&self) -> MegaSena {
        let mut rng = rand::rng();
        let mut numbers: Vec<i64> = (1..=60)
            .choose_multiple(&mut rng, 6)
            .into_iter()
            .map(|n| n as i64)
            .collect();
        numbers.sort_unstable();

        let mut set = HashSet::new();
        for n in &numbers {
            set.insert(n.clone());
        }

        MegaSena {
            id: 0,
            concurso: 999999,
            data: String::from("31/12/2025"),
            bola_1: Option::from(numbers[0]),
            bola_2: Option::from(numbers[1]),
            bola_3: Option::from(numbers[2]),
            bola_4: Option::from(numbers[3]),
            bola_5: Option::from(numbers[4]),
            bola_6: Option::from(numbers[5]),
            inserted_at: String::from("Algum momento"),
            generated_by_rust: true,
            set: set.clone(),
        }
    }
}
