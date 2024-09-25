use std::io;

fn convert(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut x = String::new();
    let mut x_i32: i32 = 0;
    
    io::stdin().read_line(&mut x).expect("DEU ERRO AQUI OH DSGÇ");

    x_i32 = convert(&x);

    println!("SEU CU! {}", x_i32);
}
