use rand::Rng;
use std::{io, process, cmp::Ordering};

pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Counter {
        return Counter { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn get_value(&self) -> i32 {
         self.value
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {

    pub fn new(value: i32) -> Result<Guess, String> {
       if value < 0 || value > 100 {
        return Err(String::from("O valor deve estar entre 0 e 100"));
       } 

       return Ok(Guess { value })
    }

    pub fn generate_randon_number() -> i32 {
         return rand::thread_rng().gen_range(0..100);
    }

    pub fn play() {
        let secret_number = Guess::generate_randon_number();
        let mut counter = Counter::new();

        loop {
            let mut guess = String::new();
            println!("Digite um numero: ");

            match io::stdin().read_line(&mut guess) {
                Ok(_) => {
                    println!("O valor digitado foi digitado e de {}", guess);
                },
                Err(err) => {
                    println!("Valor errado {}", err);
                    continue;
                }
            };

            let number_guess: i32 = guess.trim().parse().expect("Escreva um numero");

            (&mut counter).increment();
            match number_guess.cmp(&secret_number) {
                Ordering::Equal => {
                    println!("Voce acertou o numero, com {} tentativas", (&mut counter).get_value());
                    Guess::init_menu();
                },
                Ordering::Greater => {
                    println!("O numero digitado e maior que o numero secreto, tente de novo");
                    continue;
                },
                Ordering::Less => {
                    println!("O numero digitado e menor que o numero secreto, tente de novo");
                    continue;
                },
            }
        }
    }

    fn init_menu(){
        println!("=========================");
        println!("Joguin de advinhacao");
        println!("=========================");
        println!("");
        println!("Aperte 1 pra jogar e 2 pra sair");

        let mut input = String::new();
         io::stdin().read_line(&mut input);

         let input_parse: i32 = input.trim().parse().expect("Escreva um numero");

         match input_parse {
            1 => {
                Guess::play();
            }
            2 => {
                process::exit(0x0100);
            },
            _ => {
                println!("Escreva 1 ou 2, seu corno!");
                Guess::init_menu();
            }

         }
    }

}