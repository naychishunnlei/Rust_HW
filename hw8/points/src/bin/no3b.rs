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
    let mut html: String = String::new();

    html.push_str("<table>");

    html.push_str("
    <tr>
    <th style=\"text-align:center\">X</th>
    <th style=\"text-align:center\">Y</th>
    </tr>
    ");
    
    let p1 = PolarPoint::new(2.82, 0.78);
    let p2 = PolarPoint::new(5., 0.92);
    let p3 = PolarPoint::new(1.41, -0.78);
    let p4 = PolarPoint::new(5., 1.57);

    let polar_list = vec![p1, p2, p3, p4];
    let cartesian_list = to_cartesian(&polar_list);

    for pt in cartesian_list {
        html.push_str(&format!("
        <tr>
        <td style=\"text-align:right\">{:.2}</td>
        <td style=\"text-align:right\">{:.2}</td>
        </tr>
        ",pt.x, pt.y))
    }

    html.push_str("
    </table>
    ");

    println!("{}", html);
}