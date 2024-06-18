use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\tAdvinhando um número!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\tDigite um número: ");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("\tFalha ao ler a linha!");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nVocê chutou: {}", num);

        match num.cmp(&secret_num) {
            Ordering::Less => println!("Muito abaixo!"),
            Ordering::Equal => {
                println!("\tVocê ganhou!\n");
                break;
            },
            Ordering::Greater => println!("Muito acima!"),
        }
    }
}
