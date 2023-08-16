fn count_vowels(input: &str) -> usize {
    let vowels = "aeiouAEIOU";
    input.chars().filter(|&c| vowels.contains(c)).count()

}

fn main() {
    let input_string = "He11o wOrld";
    let vowels = count_vowels(input_string);
    println!("Number of vowels: {}", vowels);
}

#[test]
fn test_vowels_count1(){
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
}