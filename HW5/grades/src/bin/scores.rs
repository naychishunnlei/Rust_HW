//to run cargo run --bin scores

fn get_grade(score: u32) -> &'static str {
    match score {
        0..=49 => "F",
        50..=60 => "D",
        61..=70 => "C",
        71..=80 => "B",
        81..=94 => "A",
        95..=100 => "A+",
        _ => "Invalid score",
    }
}

fn split_scores(scores: Vec<u32>) -> (Vec<(&'static str, u32)>, Vec<(&'static str, u32)>) {
    let mut above_d = Vec::new();
    let mut d_and_f = Vec::new();

    for score in scores {
            if get_grade(score) != "Invalid score" {
            let grade = get_grade(score);

            match grade {
                "A+" | "B" | "C" =>  above_d.push((grade, score)),
                "D" | "F" => d_and_f.push((grade, score)),
                _ => {}
                
            }
        }
    }

    (above_d, d_and_f)
}

fn main() {
    let scores = vec![75, 42, 98, 54, 63];
    let (above_d, d_and_f) = split_scores(scores.clone());

    println!("Scores: {:?}", scores);
    println!("Above D: {:?}", above_d);
    println!("D and F: {:?}", d_and_f);
}

#[test]
fn test_split_scores() {
    assert_eq!(
        split_scores(vec![75, 42, 98, 54, 63]),
        (
            vec![("B", 75), ("A+", 98), ("C", 63)],
            vec![("F", 42), ("D", 54)]
        )
    );

    assert_eq!(
        split_scores(vec![95, 72, 50, 81, 100]),
        (
            vec![("A+", 95), ("B", 72), ("A", 81), ("A+", 100)],
            vec![("D", 50)]
        )
    );

    //testing with invalid scores
    assert_eq!(
        split_scores(vec![30, 68, 77, 90, 101]),
        (
            vec![("C", 68), ("B", 77), ("A", 90)],
            vec![("F", 30)]
        )
    );
}
