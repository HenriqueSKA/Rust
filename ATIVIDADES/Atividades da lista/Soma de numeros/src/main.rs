//faça um codigo que que receba um numero inteiro positivo e retorne a soma dos mesmos exemplo se o numero inserido for
//22 ele deve retornar 4
use std::io;

fn convert(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main(){

    let mut soma= 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler valor_entrada");
    let mut valor_i32 = convert(&valor_entrada);

    while valor_i32 != 0 {
        
        let mut r = valor_i32 %10;
        soma= soma + r;
        valor_i32 = valor_i32 /10;
        
    }
    println!("O valor da soma dos digitos {}", soma );

}