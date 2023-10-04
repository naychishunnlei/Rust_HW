//1.1
fn vflip(img: &[String]) -> Vec<String> {
    let mut result = img.to_vec();
    result.reverse();
    result
}

//1.2
fn hflip(img: &[String]) -> Vec<String> {
    let max_length = img.iter().map(|s| s.len()).max().unwrap_or(0);

    img.iter().map(|line| {
        let space = " ".repeat(max_length - line.len());
        format!("{}{}", space, line.chars().rev().collect::<String>())
    }).collect()
}

fn main() {
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    println!("{:?}", data);

    println!("vertical flip: {:?}", vflip(&data));
    println!("horizontal flip: {:?}", hflip(&data));

}

#[test]
fn test_img_flip() {
let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);

let data = [
    "<--",
    "#####",
    "<=="
    ].map(|v| v.to_string());

    assert_eq!(
        vflip(&data), [
            "<==",
            "#####",
            "<--"
        ]);

    assert_eq!(
        hflip(&data), [
            "  --<",
            "#####",
            "  ==<"
    ]);
}