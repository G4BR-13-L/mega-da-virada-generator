use crate::domain::mega_sena::MegaSena;

#[derive(Debug, Clone)]
pub struct SomaRange {
    pub min: u16,
    pub max: u16,
}

impl SomaRange {
    pub fn new(historico: &Vec<MegaSena>) -> SomaRange {
        let mut soma_minima: u16 = 346;
        let mut soma_maxima: u16 = 0;

        for j in historico {
            if j.generated_by_rust {
                continue;
            }
            let soma: u16 = j.bola_1.unwrap_or(0) as u16
                + j.bola_2.unwrap_or(0) as u16
                + j.bola_3.unwrap_or(0) as u16
                + j.bola_4.unwrap_or(0) as u16
                + j.bola_5.unwrap_or(0) as u16
                + j.bola_6.unwrap_or(0) as u16;
            if soma < soma_minima {
                soma_minima = soma;
            }
            if soma > soma_maxima {
                soma_maxima = soma;
            }
        }
        println!("\n\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Soma minima {}", soma_minima);
        println!("Soma maxima {}", soma_maxima);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        SomaRange {
            min: soma_minima,
            max: soma_maxima,
        }
    }

    pub fn contains(&self, soma: u16) -> bool {
        soma > self.min && soma < self.max
    }
}


#[derive(Debug, Clone)]
pub struct FaixaEstatistica {
    pub min: i64,
    pub max: i64,
    pub media: f64,
    pub desvio_padrao: f64,
    pub k: f64,
}

impl FaixaEstatistica {
    pub fn intervalo_valido(&self) -> (i64, i64) {
        let inferior = (self.media - self.k * self.desvio_padrao).floor() as i64;
        let superior = (self.media + self.k * self.desvio_padrao).ceil() as i64;
        (inferior.max(0), superior.min(6))
    }
}
