fn extract_recursive(input: &str, result: &mut Vec<String>) -> Vec<String> {
    if let Some((word, rest)) = input.split_once(" ") {
        let first_star = word.chars().next().unwrap_or('_');
        let sec_star = word.chars().rev().next().unwrap_or('_');

        if first_star == '*' && sec_star == '*' {
            let (_, word) = word.split_at(1);
            let (word, _) = word.split_at(word.len() - 1);

            result.push(word.to_string());
        }

        let v = extract_recursive(rest, result);
        return v;
    }
    else {
        let v = Vec::<String>::new();
        return v;
    }
}
  

fn main() {
    let input = "C ** *C++* *Java *Python* Rust*      ";
    let mut result = Vec::new();
    extract_recursive(input, &mut result);

    println!("Result of quote: {:?}", result);
}

#[test]
fn test_extract_recursive() {
    let input = "C ** *C++* *Java *Python* *Rust*      ";
    let mut result = Vec::new();
    extract_recursive(input, &mut result);

    let expected_result = vec!["", "C++", "Python", "Rust"];
    assert_eq!(result, expected_result);
}
