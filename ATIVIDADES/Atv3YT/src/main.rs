// 
use std::io;

fn convert(data_input: &String) -> i32{
   let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn main(){
    let mut medias  =String::new();
    println!("digite quantas medias vc vai inserir");
    io::stdin().read_line(&mut medias).expect("Erro ao ler medias");
    let mut soma_rec = 0;
    let mut i = 0;
    while convert(&medias) > i{
        let mut media_aluno = String::new();
        println!("digite as medias");
        io::stdin().read_line(&mut media_aluno).expect("Erro ao ler media_aluno");
        i += 1;
        if convert(&media_aluno) >= 3 && convert(&media_aluno) < 6 {
            soma_rec += 1;

        }
    }
    println!("O numero de alunos em rec eh {}", soma_rec);

}