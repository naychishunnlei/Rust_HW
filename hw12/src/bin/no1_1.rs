enum Shape {
    Circle {
        x: i32,
        y: i32,
        radius: f32,
    },

    Rectangle {
        x: i32,
        y: i32,
        width: f32,
        height: f32,
    },
    Triangle {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    },
}

impl Shape {
    fn rep_string(&self) -> String {
        match self {
            Shape::Circle{x, y, radius} => format!("<Circle: {}, {}, {}>", x, y, radius),
            Shape::Rectangle{x, y, width, height} => format!("<Rectangle: {}, {}, {}, {}>", x, y, width, height),
            Shape::Triangle{x1, y1, x2, y2, x3, y3} => format!("<Triangle: {}, {}, {}, {}, {}, {}>", x1, y1, x2, y2, x3, y3),
        }
    }
    fn area(&self) -> f32 {
        match self {
            Shape::Circle {radius, ..} => 3.14 * radius * radius,
            Shape::Rectangle {width, height, ..} => width * height,
            Shape::Triangle {x1, y1, x2, y2, x3, y3} => 0.5 * (((x1-x3) * (y2-y1)) - ((x1-x2) * (y3-y1))),
        }
    }
}

fn main() {
    let circle1 = Shape::Circle {
        x: 10,
        y: 10,
        radius: 30.0,
    };
    println!("{}", circle1.rep_string());
    println!("{}", circle1.area());

    let rectangle1 = Shape::Rectangle {
        x: 10,
        y: 10,
        width: 15.,
        height: 20.,
    };
    println!("{}", rectangle1.rep_string());
    println!("{}", rectangle1.area());

    let triangle1 = Shape::Triangle {
        x1: -5.,
        y1: 2.,
        x2: 3.,
        y2: 0.,
        x3: 0.,
        y3: 5.,
    };
    println!("{}", triangle1.rep_string());
    println!("{}", triangle1.area());
}

#[allow(dead_code)]
const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle { x: 0, y: 0, radius: 1.0 },
    Shape::Circle { x: 50, y: 50, radius: 15.0 },
    Shape::Rectangle { x: 40, y: 40, width: 20.0, height: 20.0 },
    Shape::Rectangle { x: 10, y: 40, width: 15.0, height: 10.0 },
    Shape::Triangle { x1: -2., y1: 1., x2: 3., y2: 2., x3: 1., y3: 5. }
];

#[allow(dead_code)]
const EXPECTED: &[&str] = &[
"<Circle: 0, 0, 1>, area: 3.14",
"<Circle: 50, 50, 15>, area: 706.50",
"<Rectangle: 40, 40, 20, 20>, area: 400.00",
"<Rectangle: 10, 40, 15, 10>, area: 150.00",
"<Triangle: -2, 1, 3, 2, 1, 5>, area: 8.50"
];

#[test]
fn test_shapes() {
    let input_list = INPUT_SHAPES;
    let shape_list = input_list.clone();
    let omap = shape_list.iter().map(
    |s| format!("{}, area: {:.2}", s.rep_string(), s.area()) );
    let output: Vec<_> = omap.collect();
    assert_eq!(output, EXPECTED);
}