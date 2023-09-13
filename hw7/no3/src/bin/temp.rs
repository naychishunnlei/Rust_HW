//to run cargo run --bin temp <start> <end> <step> > temp.html
fn fahr2cel(fahr: i32) -> f32 {
    (5.0 / 9.0) * (fahr as f32 - 32.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let start: i32 = args.get(1).and_then(|arg| arg.parse().ok()).unwrap_or(0);
    let end: i32 = args.get(2).and_then(|arg| arg.parse().ok()).unwrap_or(0);
    let step: usize = args.get(3).and_then(|arg| arg.parse().ok()).unwrap_or(0);

    let mut html = String::new();

    //Table start
    html.push_str("<table>");

    //table head
    html.push_str("
    <tr>
    <th style=\"text-align:center\">Fahr</th>
    <th style=\"text-align:center\">Celsius</th>
    </tr>
    ");

    //table body
    let (start, end) = if start <= end { (start,end) } else { (end, start) };
    for fahr in (start..=end).step_by(step) {
        let cel: f32 = fahr2cel(fahr);
        html.push_str(&format!("
        <tr>
        <td style=\"text-align:right\">{}
        </td>
        <td style=\"text-align:right\">{:.1}
        </td>
        </tr>", fahr, cel));
    }

    //table end
    html.push_str("</table>");

    println!("{}", html);
}