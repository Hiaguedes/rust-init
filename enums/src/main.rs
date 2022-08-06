use rand::Rng;


#[derive(Debug)]
enum Status {
    Success,
    Error,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Write(String),
    Color(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn enums(){
    let status = Status::Success;
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let message = Message::Write(String::from("Ola"));
    
    message.call(); // Escrevera Write("Ola"), pois e o que o Message esta implementando
    
    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i32> = None;

}

#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(coin: Coin) -> i8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }
}

fn plus_one_enum(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(number) => Some(number + 1)
    }
}

fn dice_roll(value: i8){

    fn coloca_chapeu() {
        println!("^")
    }

    fn tira_chapeu(){
        println!("x^");
    }

    fn roda_de_novo(value: i8) {
        println!("Numero colocado igual a {} ", value);
        dice_roll(rand::thread_rng().gen_range(1..7));
    }

    match value {
        2 => coloca_chapeu(),
        6 => tira_chapeu(),
        _ => roda_de_novo(value),
    }
}


fn main() {
    enums();

    println!("{}cents", Coin::value_in_cents(Coin::Nickel));
    println!("{}cents", Coin::value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);

    let five_plus_one = plus_one_enum(five);

    println!("{}", five_plus_one.unwrap()); // unwrap tira o valor de dentro do option

    dice_roll(6);
    dice_roll(1);

    let coin = Coin::Nickel;

    if let Coin::Nickel = coin { // funciona que nem o match e tem uma verbosidade menor
        println!("Dois enums iguais")
    }
}
