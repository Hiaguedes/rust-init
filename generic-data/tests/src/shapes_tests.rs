#[cfg(test)]
mod shapes_tests {
    use tests::shapes::{Point, Rectangle};

    #[test]
    pub fn calcula_area() {
        let rect = Rectangle { width: 10.0, height: 5.0};

        assert_eq!(rect.area() , 50.0);
    }

    #[test]
    pub fn calcula_ponto_origem(){
       let rect = Rectangle { width: 1.0, height: 1.0 };

       assert_eq!(rect.get_point_center(Point { x: 1.0, y: 1.0 }), Point { x: 1.5, y: 1.5 }, "O ponto retornado nao corresponde");
    }

}