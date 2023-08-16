fn count_vowels_v2(input: &str) -> Vec<(String, usize)> {
    let mut vowels_count = Vec::new();
    let vowels_input = "aeiouAEIOU";

    let words: Vec<&str> = input.split_whitespace().collect();

    for item in words {
        let vowel_count = item.chars().filter(|&c| vowels_input.contains(c)).count();

        vowels_count.push((item.to_string(), vowel_count));
    }

    vowels_count
}

fn main() {
    let input = "He11o world testing";
    let vowels_count2 = count_vowels_v2(input);

    println!("Input string: {}", input);

    for (item, vowel_count) in vowels_count2 {
        println!("Word: {}, Number of vowels: {}", item, vowel_count);
    }
}

#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
            ("7x8U3y5z".to_string(), 1) // 'U'
        ]
    );
    assert_eq!(
        count_vowels_v2("Hello World"),
        [
            ("Hello".to_string(), 2), // 'e', 'o'
            ("World".to_string(), 1) // 'o'
        ]
    );
    assert_eq!(
        count_vowels_v2("Testing 123"),
        [
            ("Testing".to_string(), 2), // 'e', 'i'
            ("123".to_string(), 0) // No vowels
        ]
    );
    assert_eq!(
        count_vowels_v2("AEiou aeiou"),
        [
            ("AEiou".to_string(), 5), // All uppercase vowels
            ("aeiou".to_string(), 5) // All lowercase vowels
        ]
    );
}