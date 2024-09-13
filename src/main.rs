use std::io::{self, Write};

fn main() {
    // COLETA O NOME
    print!("Digite o nome completo: ");
    io::stdout().flush().unwrap(); 
    let mut nome_completo = String::new();
    io::stdin().read_line(&mut nome_completo).expect("Falha ao ler a linha");
    let nome_completo = nome_completo.trim();


    // CORRIGE O NOME
    let nome_corrigido = nome_completo
        .split_whitespace() // DIVIDE OS NOMES CADA VEZ QUE ENCONTRAR UM ESPAÇO
        .map(|palavra| {
            let mut chars = palavra.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
            }
        })
        // JUNTA TODAS AS PARTES
        .collect::<Vec<String>>()
        .join(" ");

    // INVERTE APENAS A ORDEM DOS NOMES 
    let nomes_invertidos = nome_corrigido
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ");

    // INVERTE TODAS AS LETRAS DA STRING
    let nome_completamente_invertido = nome_corrigido
        .chars()
        .rev()
        .collect::<String>();

    // RESULTADOS
    println!("\nNome corrigido: {}", nome_corrigido);
    println!("Nome invertido: {}", nomes_invertidos);
    println!("Nome completamente invertido: {}", nome_completamente_invertido);
}
