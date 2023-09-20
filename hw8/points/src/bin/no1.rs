struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point {
            x,
            y,
        }
    }
}

struct PolarPoint {
    r: f32,
    t: f32,
}

impl PolarPoint {
    fn new(r: f32, t: f32) -> PolarPoint {
        PolarPoint {
            r,
            t,
        }
    }
}

fn to_polar(pt_list: &[Point]) -> Vec<PolarPoint> {
    let mut polar_points = Vec::new();
    for pt in pt_list {
        let r = (pt.x.powi(2) + pt.y.powi(2)).sqrt();
        let t = pt.y.atan2(pt.x);
        polar_points.push(PolarPoint {r,t});
    }
    polar_points
}

fn to_cartesian(pt_list: &[PolarPoint]) -> Vec<Point> {
    let mut cartesian_points = Vec::new();
    for pt in pt_list {
        let x = pt.r * pt.t.cos();
        let y = pt.r * pt.t.sin();
        cartesian_points.push(Point {x,y});
    }
    cartesian_points
}

fn main() {
    let p1 = Point::new(1., 1.);
    let p2 = Point::new(2., 4.);
    let p3 = Point::new(-1., 0.);
    let p4 = Point::new(3., -5.);

    let pt_list = vec![p1, p2, p3, p4];
    let polar_list = to_polar(&pt_list);

    println!("1.1\nCartesian Points");
    for pt in pt_list {
        println!("{}, {}", pt.x, pt.y);
    }

    println!("Polar Points");
    for pt in polar_list {
        println!("{}, {}", pt.r, pt.t);
    }
    println!();

    let polar1 = PolarPoint::new(2.82, 0.78);
    let polar2 = PolarPoint::new(5.0, 0.92);
    let polar3 = PolarPoint::new(1.41, -0.78);
    let polar4 = PolarPoint::new(5.0, 1.57);
    
    let polar_list = vec![polar1, polar2, polar3, polar4];
    let cartesian_list = to_cartesian(&polar_list);

    println!("No 1.2\nPolar Points");
    for pt in polar_list {
        println!("{}, {}", pt.r, pt.t);
    }

    println!("Cartesian Points");
    for pt in cartesian_list {
        println!("{}, {}", pt.x, pt.y);
    }    
}