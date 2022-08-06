pub enum AppleColor {
    Green,
    Red,
}

pub struct Apple {
    pub color: AppleColor,
    pub name: String,
}

impl Apple {
    pub fn get_name(&self) -> &String{
        return &self.name;
    }
}

pub mod apple {
    pub fn print_apple(){
        println!("Eu sou uma maca");
    }

}