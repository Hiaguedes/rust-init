pub mod shapes_tests;
pub mod shapes;
pub mod guess;
pub mod guess_test;

use guess::Guess;
fn main() {
    Guess::play();
}
