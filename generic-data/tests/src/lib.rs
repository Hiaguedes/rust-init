pub mod shapes;
pub mod guess;

mod math {
    fn add(x: i32, y: i32) -> i32 {
        return x + y;
    }

    #[test]
    fn test_add() {
        assert_eq!(add(1, 1), 2);
        assert_eq!(add(1, -1), 0);
        assert_eq!(add(2, 2), 4);
    }
}