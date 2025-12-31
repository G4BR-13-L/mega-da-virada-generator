use crate::domain::mega_sena::MegaSena;
use crate::domain::heuristics::FaixaEstatistica;

pub fn calcular_faixa_estatistica(
    historico: &[MegaSena],
    min: i64,
    max: i64,
    k: f64,
) -> FaixaEstatistica {
    let mut contagens = Vec::with_capacity(historico.len());

    for jogo in historico {
        let count = jogo
            .set
            .iter()
            .filter(|&&n| n >= min && n <= max)
            .count() as usize;

        contagens.push(count as f64);
    }

    let media = contagens.iter().sum::<f64>() / contagens.len() as f64;

    let variancia = contagens
        .iter()
        .map(|v| (v - media).powi(2))
        .sum::<f64>()
        / contagens.len() as f64;

    let desvio_padrao = variancia.sqrt();

    FaixaEstatistica {
        min,
        max,
        media,
        desvio_padrao,
        k,
    }
}
