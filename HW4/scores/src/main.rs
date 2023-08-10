fn grade_checker(score: f32) -> &'static str {
    if 0.0 <= score && score <= 49.0 { return "Failed with F"; }
    else if 50.0 <= score && score <= 60.0 { return "D"; }
    else if 61.0 <= score && score <= 70.0 { return "C"; }
    else if 71.0 <= score && score <= 80.0 { return "B"; }
    else if 81.0 <= score && score <= 94.0 { return "A"; }
    else if 95.0 <= score && score <= 100.0 { return "Excellent with A+"; }
    else { return "Invalid score"; }
}

// Using ordinary loop
fn make_grades_loop(scores: Vec<f32>) -> Vec<&'static str> {
    let mut grades = Vec::new();

    for score in scores {
        let grade = grade_checker(score);
        grades.push(grade);
    }

    grades
}

// Using recursion
fn make_grades_recursion(scores: Vec<f32>, grades: &mut Vec<String>, index: usize) {
    if index < scores.len() {
        grades.push(grade_checker(scores[index]).to_string());
        make_grades_recursion(scores, grades, index + 1);
    }
}

#[test]
fn test_make_grades() {
    let test_scores = vec![75.0, 92.5, 40.0, 105.0];
    let expected_grades = vec!["B", "A", "Failed with F", "Invalid score"];
    let actual_grades = make_grades_loop(test_scores.clone());
    let mut recurr_grades = Vec::new();
    make_grades_recursion(test_scores.clone(), &mut recurr_grades, 0);


    //test for ordinary loop
    assert_eq!(expected_grades, actual_grades);

    //test for recurrsion
    assert_eq!(expected_grades, recurr_grades);
}

#[test]
fn test_make_grades_without_values() {
    let test_scores = vec![];
    let expected_grades: Vec<String> = vec![];
    let actual_grades = make_grades_loop(test_scores.clone());
    let mut recurr_grades = Vec::new();
    make_grades_recursion(test_scores.clone(), &mut recurr_grades, 0);

    //test for ordinary loop
    assert_eq!(expected_grades, actual_grades);

    //test for recurrsion
    assert_eq!(expected_grades, recurr_grades);
}

#[test]
fn test_make_grades_invalid_values() {
    let test_scores = vec![-1.0, 125.0, 60.0];
    let expected_grades = vec!["Invalid score", "Invalid score", "D"];
    let actual_grades = make_grades_loop(test_scores.clone());
    let mut recurr_grades = Vec::new();
    make_grades_recursion(test_scores.clone(), &mut recurr_grades, 0);

    //test for ordinary loop
    assert_eq!(expected_grades, actual_grades);

    //test for recurrsion
    assert_eq!(expected_grades, recurr_grades);
}


fn main() {
    let test_scores = vec![75.0, 92.5, 40.0, 105.0];

    // Using ordinary loop
    let grades_loop = make_grades_loop(test_scores.clone());
    println!("Using ordinary loop:");
    for (i, grade) in grades_loop.iter().enumerate() {
        println!("Score {}: Grade: {}", test_scores[i], grade);
    }

    // Using recursion
    let mut grades_recursion = Vec::new();
    make_grades_recursion(test_scores.clone(), &mut grades_recursion, 0);
    println!("Using recursion:");
    for (i, grade) in grades_recursion.iter().enumerate() {
        println!("Score {}: Grade: {}", test_scores[i], grade);
    }
}
