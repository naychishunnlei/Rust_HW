use std::io::Read;
use std::io::Write;
use std::fs::File;

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

//2.1
fn read_cartesian_file(reader: impl Read) -> Vec<Point> {
    let mut points = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(reader);

    for record in reader.records() {
        let record = record.unwrap();
        if record.len() < 2 {
            continue;
        }
        let x: f32 = record[0].parse().unwrap_or(0.0);
        let y: f32 = record[1].parse().unwrap_or(0.0);
        let point = Point::new(x, y);
        points.push(point);
    }
    points
}

fn save_polar_file(writer: impl Write, points: &[PolarPoint]) {
    let mut writer = csv::WriterBuilder::new()
                    .has_headers(false)
                    .delimiter(b',')
                    .from_writer(writer);

    for point in points {
        writer.serialize((point.r, point.t)).unwrap();
    }
    writer.flush().unwrap();
}

//2.2
fn read_polar_file(reader: impl Read) -> Vec<PolarPoint> {
    let mut points = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(reader);
    
    for record in reader.records() {
        let record = record.unwrap();
        if record.len() < 2 {
            continue;
        }
        let r: f32 = record[0].parse().unwrap_or(0.0);
        let t: f32 = record[1].parse().unwrap_or(0.0);
        let point = PolarPoint::new(r, t);
        points.push(point);
    }
    points
}

fn save_cartesian_file(writer: impl Write, points: &[Point]) {
    let mut writer = csv::WriterBuilder::new()
                    .has_headers(false)
                    .delimiter(b',')
                    .from_writer(writer);

    for point in points {
        writer.serialize((point.x, point.y)).unwrap();
    }
    writer.flush().unwrap();
}

fn main() {
     // 2.1
     let cartesian_input_file = File::open("./cartesian_input.csv").expect("Unable to open the file");
     let cartesian_list = read_cartesian_file(cartesian_input_file);
     let polar_list = to_polar(&cartesian_list);
     let polar_output_file = File::create("./polar_output.csv").expect("Unable to create the file");
     save_polar_file(polar_output_file, &polar_list);
 
     // 2.2
     let polar_input_file = File::open("./polar_input.csv").expect("Unable to open the file");
     let polar_list = read_polar_file(polar_input_file);
     let cartesian_list = to_cartesian(&polar_list);
     let cartesian_output_file = File::create("./cartesian_output.csv").expect("Unable to create the file");
     save_cartesian_file(cartesian_output_file, &cartesian_list);
}