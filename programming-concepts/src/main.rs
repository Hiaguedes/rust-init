use std::cmp::Ordering;

fn variables_mutuabilty() {
    let mut x = 5; // somente let x = 5; daria erro na linha 4 onde digo que x=6
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    const TWO_HOURS_SECONDS: i32 = 60 * 60 * 2;
    
    println!("{TWO_HOURS_SECONDS}s")

}

fn shadowing() {
    let x = 5;

    let x = x + 1; // por que posso declarar de novo a variavel pegando o valor da anterior

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // o valor do x dentro desse escopo fechado que fecha logo em seguida
    }

    println!("The value of x is: {x}");

    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num;
    // se a variavel ta mut entao eu nao posso mudar o tipo da variavel
}

fn typing() {
    let guess: u32 = "32".parse().expect("Not a number"); //parse converte de string pra number

    println!("{guess}");
}

fn float(){
    let y: f32 = 3.0;
    println!("O valor de y: {y}");
    let y: f32 = 3.7;
    println!("O valor de y: {y}");
    // let y: f32 = 5;  da erro
}

fn operacoes_matematicas(){
    let sum = 5+10;
    let sub = 5-10;
    let mult = 5.1 * 6.1;
    let division = 3/2; // as divisoes naturalmente arredondam pro tipo inteiro
    let division_floor = 2 / 3;
    let division_float: f32 = 3 as f32 /2 as f32; // as divisoes naturalmente arredondam pro tipo inteiro
    let division_float_second:f32 = 2 as f32 / 3 as f32;
    let rest = 7%9;
    
    println!("sum: {sum}, sub: {sub}, mult: {mult}, division: {division}, division_floor: {division_floor}, rest: {rest}");
    println!("division_float: {division_float}, division_float_second: {division_float_second}")
}

fn bool(){
    let t = true;
    let f = false;

    println!("{}", t == f);

}

fn char(){
    let letter_a = 'a';
    let letter_A = 'A';

    match letter_a.cmp(&letter_A) {
        Ordering::Equal => println!("Iguais"), 
        Ordering::Less => println!("segundo maior"), 
        Ordering::Greater => println!("primeiro maior"), 
    }
}

fn tupla_array() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, _y ,_z) = tup;

    let arr = [1, 2, 3];
    println!("x: {x} da tupla");
    let [x, _y, _z] = arr;
    println!("x: {x} do array");

    let arr1 = [0; 5]; // modo de termos um array com 5 elementos iguais a 0
    println!("Array {:?}", arr1);
}

fn foo(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}")
}

fn five() -> i32 {
     5 // posso suprimir o return 
}

fn is_even(x: i32) -> bool {
    x%2 == 0
}

fn if_statement(number: i32){

    if number < 3 {
        println!("Menor que 3")
    } else {
        println!("Maior que 3")
    }
}

fn loop_repition(){
    let mut a = 0;

    loop {
        println!("Estou em loop");
        if a == 5 {
            break;
        }
        a += 1;
    }
}

fn return_loop() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 7; // fara result ser igual a 70
        }
    };

    println!("resultado e {result}");
}

fn for_loop(){
    for number in (1..4).rev(){
        println!("{number}")
    }

    println!("End for loop");
}

fn while_loop(){
    let mut number = 0;

    while number !=5 {
        println!("{number}");
        number += 1;
    }

    println!("End while loop");
}

fn loop_inside_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn main() {
    variables_mutuabilty();
    shadowing();
    typing();
    float();
    operacoes_matematicas();
    bool();
    char();
    tupla_array();
    foo();
    println!("{}", five());
    println!("3 e par? {}", is_even(3));
    println!("4 e par? {}", is_even(4));
    if_statement(5);
    {
        let condition = true;
        let number = if condition { 5 } else { 6 };
    
        println!("The value of number is: {number}");
    }
    loop_repition();
    return_loop();
    loop_inside_loop();
    for_loop();
    while_loop();
    println!("{:?}", (1..5));
}
