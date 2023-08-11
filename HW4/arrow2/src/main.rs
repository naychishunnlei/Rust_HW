fn make_arrow2(size: i32) -> String {
    let mut arrow = String::new();
    
    // Loop for upper half of arrow
    for i in (1..size).rev() {
        // Print spaces
        for _j in 1..=i {
            arrow.push(' ');
        }
        // Print stars
        for _k in i..size {
            arrow.push('*');
        }
        arrow.push('\n');
    }
    
    // Loop for lower half of arrow
    for i in 0..size {
        // Print spaces
        for _j in 0..i {
            arrow.push(' ');
        }
        // Print stars
        for _k in i..size {
            arrow.push('*');
        }
        arrow.push('\n');
    }
    
    arrow
}

fn main() {
    let size = 4; 
    
    let arrow2 = make_arrow2(size);
    print!("{}", arrow2);

}

#[test]
fn test_make_arrow2() {
    let star = 4;
    let expected = "   *\n  **\n ***\n****\n ***\n  **\n   *\n";
    let arrow = make_arrow2(star);

    assert_eq!(expected, arrow);
}
