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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size_arg = if args.len() < 2 {""} else {&args[1]};
    let size: i32 = size_arg.parse().unwrap_or(0);

    let arrow1 = make_arrow1(size);
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

