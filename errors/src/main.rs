use core::panic;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{ErrorKind, Read};
use rand::Rng;

fn open_file_match(){

    let f = File::open("hello.txt");
    
    // match f {
    //     Ok(file) => file,
    //     Err(err) => panic!("Problema em abrir o arquivo {:?}", err),
    // };
    
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(file_created) => file_created,
                Err(err) => panic!("Problem creating the file {:?}", err)
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        }
    };
    
    println!("Chegaste aqui");
}

fn open_file_fn() -> Result<String, std::io::Error> {
    // let mut f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    let f = File::open("./hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn guess_number(guess: i32) {
    let secret_number = rand::thread_rng().gen_range((0..100));
    loop {
        if guess < 1 || guess > 100 {
            println!("O numero digitado deve estar entre 1 e 100");
            panic!("O valor deve estar entre 1 e 100");
            break;
        }

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Voce acertou");
                break;
            },
            Ordering::Less => {
                println!("ERROU! o Valor e maior");
                continue;
            },
            Ordering::Greater => { 
                println!("ERROU! O valor e menor");
                continue;
             }
        }

    }
}

fn main() {
    // panic!("Pane no sistema alguem me desconfigurou");
    // println!("{:?}", open_file_fn().unwrap());
    guess_number(-10);
}
