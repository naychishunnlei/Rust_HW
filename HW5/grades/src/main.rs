fn split_grades(grades: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut above_d = Vec::new();
    let mut d_and_f = Vec::new();

    for grade in grades {
        match grade {
            "A" | "A+" | "A-" | "B" | "B+" | "B-" | "C" | "C+" | "C-" => above_d.push(grade),
            "D" | "F" => d_and_f.push(grade),
            _ => {} // Ignore other grades
        }
    }

    (above_d, d_and_f)
}

fn main() {
    let grades = vec!["B", "F", "A+", "D", "C"];
    let (above_d, d_and_f) = split_grades(grades.clone());

    println!("Grades: {:?}", grades);
    println!("Above D: {:?}", above_d);
    println!("D and F: {:?}", d_and_f);
}

#[test]
fn test_split_grades() {
    assert_eq!(
        split_grades(vec!["B", "F", "A+", "D", "C"]),
        (vec!["B", "A+", "C"], vec!["F", "D"])
    );

    assert_eq!(
        split_grades(vec!["A", "B-", "C+", "D", "F"]),
        (vec!["A", "B-", "C+"], vec!["D", "F"])
    );

    assert_eq!(
        split_grades(vec!["C", "C-", "D", "F"]),
        (vec!["C", "C-"], vec!["D", "F"])
    );

    assert_eq!(
        split_grades(vec!["A", "A+", "A-", "B", "B+", "B-"]),
        (vec!["A", "A+", "A-", "B", "B+", "B-"], vec![])
    );

    assert_eq!(
        split_grades(vec!["D", "D", "D", "D", "D"]),
        (vec![], vec!["D", "D", "D", "D", "D"])
    );
}

