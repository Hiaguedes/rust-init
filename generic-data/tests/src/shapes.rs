pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        return self.width * self.height;
    }

    pub fn get_point_center(&self, origin: Point) -> Point {
       return Point {
        x: origin.x + self.width / 2.0,
        y: origin.y + self.height / 2.0,
       }
    }
}

pub mod shapes {

}