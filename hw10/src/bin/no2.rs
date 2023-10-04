fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = img1.to_vec();
    
    for line in img2 {
        result.push(line.clone());
    }
    result
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let max_length = img1.iter()
                    .map(|s| s.len())
                    .chain(img2.iter().map(|s| s.len()))
                    .max()
                    .unwrap_or(0);

    let mut result = Vec::new();

    for (line1, line2) in img1.iter().zip(img2.iter()) {
        let space = " ".repeat(max_length - line1.len());
        let combined_line = format!("{}{}{}", line1, space, line2);
        result.push(combined_line);
    }
    if img1.len() > img2.len() {
        let combined_line = format!("{}", img1[img1.len()-1]);
        result.push(combined_line);
    }
    else if img1.len() < img2.len() {
        let space = " ".repeat(max_length);
        let combined_line = format!("{}{}", space, img2[img2.len() - 1]);
        result.push(combined_line)
    }
    result
}

fn main(){
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    println!("{:?}", &data);

    println!("vertical flip: {:?}", vcat(&data, &data));
    println!("horizontal flip: {:?}", hcat(&data, &data));

}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);

    let data = [
        "<--",
        "#####",
        "<=="
    ].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);

    assert_eq!(
        vcat(&data, &data), [
            "<--",
            "#####",
            "<==",
            "<--",
            "#####",
            "<=="
        ]);

    assert_eq!(
        hcat(&data, &data[..2]), [
            "<--  <--",
            "##########",
            "<=="
        ]);

    assert_eq!(
        hcat(&data[..2], &data), [
            "<--  <--",
            "##########",
            "     <=="
        ]);
}