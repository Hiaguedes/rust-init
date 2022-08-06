struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle  {
    width: i32,
    height: i32,
}

struct Circle {
    ratio: i32,
}

fn reactangle_area(rect: Rectangle) -> i32 {
    return rect.height * rect.width;
}

fn circle_area(circle: Circle) -> i32 {
    return circle.ratio.pow(2);
}

impl Rectangle { // implementa methodos dentro do struct Rectangle
    fn area(&self) -> i32 { // In the signature for area, we use &self instead of rectangle: &Rectangle. The &self is actually short for self: &Self
        return self.height * self.width;
    }

    fn can_hold(&self, rect: Rectangle) -> bool {
        return self.width > rect.width && self.height > rect.height;
    }

    fn new_square(size: i32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
 {
    let user : User = User{ 
        active: true, 
        username: String::from("lol"), 
        email: String::from("wsdwd"), 
        sign_in_count: 1 
    };

    println!("{}", user.email);
 }
 println!("Area do retangulo de lados 2 e 3: {}", reactangle_area(Rectangle{height: 2, width: 3}));
 println!("Area do circulo de raio 2: {}", circle_area(Circle { ratio:2 }));

 {
    let rect: Rectangle = Rectangle { width: (20), height: (10) };
    println!("{}", rect.area());

    println!("rect pode segurar outro retangulo? {}", rect.can_hold(Rectangle {height: 5, width: 5}));

    let square = Rectangle::new_square(5); // funcao associada, ou estatica que retorna uma instancia 
    println!("Novo quadrado de lados {}, {}", square.height, square.width);
 }
}