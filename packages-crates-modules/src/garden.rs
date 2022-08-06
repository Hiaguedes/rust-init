use super::fruits::apples::apple::print_apple;

pub mod flowers {
    pub fn print_flower(){
        println!("eu sou uma flor");
    }

    pub fn print_apple_inside_flower(){
        println!("So consigo printar maca pq usei o super");
    }
}