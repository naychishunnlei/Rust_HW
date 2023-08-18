//to run cargo run --bin words

fn extract_quoted_words(input: &str) -> Vec<String> {
    let input= input.to_string();
    let input = input.split_whitespace();
    let mut result = Vec::new();

    for word in input {
        let first_star = word.chars().next().unwrap_or('_');
        let sec_star = word.chars().rev().next().unwrap_or('_');

        if first_star == '*' && sec_star == '*' {
            
            let (_, word) = word.split_at(1);
            let (word, _) = word.split_at(word.len()-1);

            result.push(word.to_string());
        }
    }

    return result;
}

fn main() {
    let input = "C ** *C++* *Java *Python* Rust* ";
    let result = extract_quoted_words(input);

    println!("Result of quote : {:?}", result);
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<String>::new() );
    assert_eq!(
    extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        ["", "C++", "Python"] 
    );
}