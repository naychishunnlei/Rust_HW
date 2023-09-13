//1.2
fn bubble_sort_ascending(numbers: &mut Vec<i32>) {
    let n = numbers.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if numbers[j] > numbers[j+1] {
                numbers.swap(j, j+1);
            }
        }
    }
}

fn bubble_sort_descending(numbers: &mut Vec<i32>) {
    let n = numbers.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if numbers[j] < numbers[j+1] {
                numbers.swap(j, j+1)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Please provide a list of numbers.");
        return;
    }

    let mut numbers: Vec<i32> = args[1..]
        .iter()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    //1.1 in ascending order
    numbers.sort_by(|a, b| a.cmp(b));
    println!("Ascending sort: {:?}", numbers);

    //1.1 in descending order
    numbers.sort_by(|a, b| b.cmp(a));
    println!("Descending sort: {:?}", numbers);

    //1.2 in ascending order
    let mut ascending_numbers = numbers.clone();
    bubble_sort_ascending(&mut ascending_numbers);
    println!("Ascending bubble_sort: {:?}", ascending_numbers);

    //1.2 in descending order
    let mut descending_numbers = numbers.clone();
    bubble_sort_descending(&mut descending_numbers);
    println!("Descending Bibble_sort: {:?}", descending_numbers);
}

#[test]
fn test_empty_input() {
    let mut numbers: Vec<i32> = vec![];

    numbers.sort_by(|a, b| a.cmp(b));
    assert_eq!(numbers, vec![]);

    numbers.sort_by(|a, b| b.cmp(a));
    assert_eq!(numbers, vec![]);

    bubble_sort_ascending(&mut numbers);
    assert_eq!(numbers, vec![]);

    bubble_sort_descending(&mut numbers);
    assert_eq!(numbers, vec![]);
}

#[test]
fn test_sorting() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];

    let mut ascending = numbers.clone();
    ascending.sort_by(|a, b| a.cmp(b));
    assert_eq!(ascending, vec![1, 1, 2, 3, 4, 5, 6, 9]);

    let mut descending = numbers.clone();
    descending.sort_by(|a, b| b.cmp(a));
    assert_eq!(descending, vec![9, 6, 5, 4, 3, 2, 1, 1]);

    bubble_sort_ascending(&mut numbers);
    assert_eq!(numbers, vec![1, 1, 2, 3, 4, 5, 6, 9]);

    bubble_sort_descending(&mut numbers);
    assert_eq!(numbers, vec![9, 6, 5, 4, 3, 2, 1, 1]);
}