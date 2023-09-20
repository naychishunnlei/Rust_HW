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

fn main() {
    let mut html: String = String::new();

    html.push_str("<table>");

    html.push_str("
    <tr>
    <th style=\"text-align:center\">Radian</th>
    <th style=\"text-align:center\">Theta</th>
    </tr>
    ");
    
    let p1 = Point::new(1., 1.);
    let p2 = Point::new(2., 4.);
    let p3 = Point::new(-1., 0.);
    let p4 = Point::new(3., -5.);

    let pt_list = vec![p1, p2, p3, p4];
    let polar_list = to_polar(&pt_list);

    for pt in polar_list {
        html.push_str(&format!("
        <tr>
        <td style=\"text-align:right\">{:.2}</td>
        <td style=\"text-align:right\">{:.2}</td>
        </tr>
        ",pt.r, pt.t))
    }

    html.push_str("
    </table>
    ");

    println!("{}", html);
}