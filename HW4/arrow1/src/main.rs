//ordinary loop
fn make_arrow1(size: i32) -> String {
    let mut arrow = String::new();

    for i in 1..=size {
        for _ in 0..i {
            arrow.push('*');
        }
        arrow.push('\n');
    }

    for i in (1..size).rev() {
        for _ in 0..i {
            arrow.push('*');
        }
        arrow.push('\n');
    }

    arrow
}

//recursion
fn make_arrow1_recur(size: i32, row_index: i32) -> String {
    if row_index >= size * 2 {
        return String::new();
    }

    let mut arrow = String::new();
    let star_index = if row_index <= size {
        row_index
    } else {
        size * 2 - row_index
    };

    star_generator(star_index, &mut arrow);
    arrow.push('\n');
    arrow.push_str(&make_arrow1_recur(size, row_index + 1));

    arrow
}

fn star_generator(star_index: i32, arrow: &mut String) {
    if star_index > 0 {
        arrow.push('*');
        star_generator(star_index - 1, arrow);
    }
}

fn main() {
    println!("Using ordinary loop:");
    let arrow1_size: i32 = 5;
    let arrow1 = make_arrow1(arrow1_size);
    print!("{}", arrow1);

    println!("Using recursion:");
    let arrow1_size: i32 = 5;
    let arrow1 = make_arrow1_recur(arrow1_size, 1);
    print!("{}", arrow1);
}


#[test]
fn test_make_arrow1() {
    assert_eq!(make_arrow1(0), "");
    assert_eq!(make_arrow1(1), "*\n");
    assert_eq!(
        make_arrow1(5),
        "*\n**\n***\n****\n*****\n****\n***\n**\n*\n"
    );
}

#[test]
fn test_make_arrow1_recur() {
    let star = 3;
    let expected = "*\n**\n***\n**\n*\n";
    let arrow = make_arrow1_recur(star, 1);

    assert_eq!(expected, arrow);
}

#[test]
fn test_arrow1_star_generator_with_zero() {
    let star_num = 0;
    let mut arrow = String::new();
    let expected = "";

    star_generator(star_num, &mut arrow);

    assert_eq!(expected, arrow);
}

