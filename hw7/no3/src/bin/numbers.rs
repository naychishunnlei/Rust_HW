//to run cargo run --bin numbers <start> <end> <step> > numbers.html
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let start: i32 = args.get(1).and_then(|arg| arg.parse().ok()).unwrap_or(0);
    let end: i32 = args.get(2).and_then(|arg| arg.parse().ok()).unwrap_or(0);
    let step: usize = args.get(3).and_then(|arg| arg.parse().ok()).unwrap_or(0);

    let mut html = String::new();

    html.push_str("<table>");

    html.push_str("
    <tr>
    <th style=\"text-align:center\">x</th>
    <th style=\"text-align:center\">x^2</th>
    <th style=\"text-align:center\">x^3</th>
    </tr>
    ");

    for x in (start..=end).step_by(step) {
        let x_squared = x * x;
        let x_cubed = x * x * x;
        html.push_str(&format!("
        <tr>
        <td style=\"text-align:center\">{}</td>
        <td style=\"text-align:center\">{}</td>
        <td style=\"text-align:center\">{}</td>
        </tr>", x, x_squared, x_cubed));
    }

    html.push_str("/table");

    println!("{}", html);
}
