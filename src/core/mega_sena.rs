use std::fmt;

#[derive(Debug, Clone)]
pub struct MegaSena {
    pub id: i64,
    pub jogo: Vec<i64>,
}

impl fmt::Display for MegaSena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formata o ID
        write!(f, "MegaSena #{}: ", self.id)?;

        // Formata os números do jogo
        if self.jogo.is_empty() {
            write!(f, "[]")
        } else {
            // Converte cada número para string e junta com vírgulas
            let numeros: Vec<String> = self.jogo.iter().map(|n| n.to_string()).collect();
            write!(f, "[{}]", numeros.join(", "))
        }
    }
}
