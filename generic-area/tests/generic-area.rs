use generic_area::*;

#[test]
fn test_generic_area() {
    let circle = Circle::new(10.0);
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    let square = Square::new(20.0);

    assert_eq!(314.1592653589793, circle.area());
    assert_eq!(6.0, triangle.area());
    assert_eq!(400.0, square.area());
    print_area(circle);
    print_area(triangle);
    print_area(square);
}
