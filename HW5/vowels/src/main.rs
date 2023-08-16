fn count_vowels(input: &str) -> usize {
    let vowels = "aeiouAEIOU";
    input.chars().filter(|&c| vowels.contains(c)).count()

}

fn count_vowels_r(input: &str) -> usize {
    if input.is_empty() {
        return 0;
    }
    else {
        let first_char = input.chars().next().unwrap();
        let rest_string = &input[1..];

        let vowels = "aeiouAEIOU";
        let is_vowel = vowels.contains(first_char);

        if is_vowel {
            1 + count_vowels_r(rest_string)
        }
        else {
            count_vowels_r(rest_string)
        }
    }
}

fn main() {
    let input_string = "He11o wOrld";
    let vowels = count_vowels(input_string);
    let vowels_r = count_vowels_r(input_string);
    println!("Number of vowels: {}", vowels);
    println!("Number of vowels using recursion: {}", vowels_r);
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("Hello World"), 3);
    assert_eq!(count_vowels("Testing 123"), 2);
    assert_eq!(count_vowels("AEiou aeiou"), 10);
}

#[test]
fn test_vowels_count_r() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
    assert_eq!(count_vowels("Hello World"), 3);
    assert_eq!(count_vowels("Testing 123"), 2);
    assert_eq!(count_vowels("AEiou aeiou"), 10);
    assert_eq!(count_vowels("Rust"), 1);
    assert_eq!(count_vowels("Programming"), 4);
}
