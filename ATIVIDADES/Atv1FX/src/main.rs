//
//
//
//
//



//use std::io;

//fn convert_to_int(data_input: &String) -> i32{
//    let x = data_input.trim().parse::<i32>().unwrap();
//    return x;
//}

//fn main() {
   
//}

use std::io;

fn main() {
    let mut numeros: Vec<i32> = Vec::new();
    
    loop {
        println!("Escolha uma opção:");
        println!("1. Adicionar número");
        println!("2. Remover último número");
        println!("3. Exibir números");
        println!("4. Sair");
        
        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");
        
        let escolha: u32 = match escolha.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match escolha {
            1 => {
                println!("Digite o número a ser adicionado:");
                let mut num = String::new();
                io::stdin().read_line(&mut num).expect("Erro ao ler entrada");
                let num: i32 = match num.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Entrada inválida");
                        continue;
                    }
                };
                numeros.push(num);
                println!("Número adicionado!");
            },
            2 => {
                if numeros.pop().is_some() {
                    println!("Último número removido!");
                } else {
                    println!("O vetor está vazio.");
                }
            },
            3 => {
                println!("Números no vetor: {:?}", numeros);
            },
            4 => {
                println!("Saindo...");
                break;
            },
            _ => {
                println!("Opção inválida!");
            }
        }
    }
}
