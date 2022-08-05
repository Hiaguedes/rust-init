use std::io; // library padrao que podem ser vistas aqui https://doc.rust-lang.org/std/prelude/index.html
mod placeholder;
use std::cmp::Ordering; // pergeito pra ordenacoes
use rand::Rng;

pub use crate::placeholder::placeholder as other_placeholder; // quanto a importacoes nao veremos isso aqui

fn adivinhando_numero_aleatorio() {
    let numero_secreto = rand::thread_rng().gen_range(1..=100); // lib externa que faz a selecao de um numero aleatorio

    loop {
    println!("Adinhe o numero secreto");

    let mut guess = String::new();

    
        io::stdin()
            .read_line(&mut guess)
            .expect("Numero nao identificado");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // faz a conversao de string pro tipo do numero_secreto que e um numero de 32 bits e com o parse conseguimos inferir pra esse tipo
        match guess.cmp(&numero_secreto) { // match faz uma comparacao e retorna algumas das condicoes que voce colocar (caso nao entre ele entram em panico com panic!)
            Ordering::Less => println!("TooSmall\n"),
            Ordering::Greater => println!("muito grande\n"),
            Ordering::Equal => { println!("yay\n"); break; },
        }
    }

}

fn adivinhador_numeros() {

    println!("Adivinhe o numero");

    let mut guess = String::new(); // guardando valores em variaveis (nao confunda let do js com let do rust)

    // o let e imutavel, colocamos ela como mutavel com o mut

    // a atribuicao String::new() e basicamente dizendo que queremos uma nova instancia de uma string como em js e const guest = String(); 

    io::stdin() // pq importamos a lib std::io no comeco se nao fariamos std::io::stdin, e ele faz uma interrupcao no codigo
        .read_line(&mut guess) // le a linha no terminal e guarda em guess, o & e uma referencia a variavel guess, no capitulo 4 se explica mais
        .expect("Failed to read line"); // read_line retorna um valor do tipo Ok ou Err e e do tipo Result e retornos do tipo 
        // Result tem um valor que pode ser lido facilmente com o metodo expect que retorno o valor de fato se for do tipo Ok e 
        // o erro passado dentro dele caso seja do tipo Err 

        // se nao possassemos o expect nos receberiamos um warning dizendo pra tratarmos o erro

        println!("You guessed: {guess}"); // placeholder string (ou no js parecido com o  template string)

}

fn main() {
    other_placeholder::func();
    adivinhador_numeros();
    adivinhando_numero_aleatorio();
}
