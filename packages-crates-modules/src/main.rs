mod garden;
// use crate::garden::flowers; //pra usar flowers::print_flower
mod fruits;
use fruits::apples::apple::print_apple;
use garden::flowers::print_apple_inside_flower;
use fruits::apples::AppleColor;
use fruits::apples::Apple; // use e um link simbolico para o arquivo

fn main() {
    garden::flowers::print_flower();
    print_apple();

    let new_apple = Apple { color: AppleColor::Red, name: String::from("Gala")  };

    println!("{}", new_apple.get_name());
    print_apple_inside_flower();
}
