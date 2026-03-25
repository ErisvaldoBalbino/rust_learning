use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("-=-=-=Tente advinhar o número=-=-=-");

    let numero_aleatorio = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Digite o seu palpite:");

        let mut palpite = String::new();

        std::io::stdin()
            .read_line(&mut palpite)
            .expect("Erro ao ler palpite");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match palpite.cmp(&numero_aleatorio) {
            Ordering::Less => println!("Abaixo."),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
            Ordering::Greater => println!("Acima."),
        }
    }
}
