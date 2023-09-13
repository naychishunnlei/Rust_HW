fn bubble_sort(pts: Vec<(f32, f32)>) -> (Vec<(f32, f32)>, Vec<(f32, f32)>) {
    let mut ascending = pts.clone();
    let mut descending = pts.clone();

    // Sort in ascending order
    for i in 0..ascending.len() {
        for j in 0..ascending.len() - i - 1 {
            if ascending[j].0 > ascending[j + 1].0
                || (ascending[j].0 == ascending[j + 1].0 && ascending[j].1 > ascending[j + 1].1)
            {
                let temp = ascending[j];
                ascending[j] = ascending[j + 1];
                ascending[j + 1] = temp;
            }
        }
    }

    // Sort in descending order
    for i in 0..descending.len() {
        for j in 0..descending.len() - i - 1 {
            if descending[j].0 < descending[j + 1].0
                || (descending[j].0 == descending[j + 1].0 && descending[j].1 < descending[j + 1].1)
            {
                let temp = descending[j];
                descending[j] = descending[j + 1];
                descending[j + 1] = temp;
            }
        }
    }

    (ascending, descending)
}

fn point(mut points: Vec<f32>) -> Vec<(f32, f32)> {
    if points.len()%2 != 0 {
        points.pop();
    }

    let mut result = Vec::new();
    for i in 0..points.len() {
        if i == 0 || i%2 == 0 {
            result.push((points[i], points[i+1]));
        }
    }
    return result;
}

fn main() {
    let vec: Vec<f32> = std::env::args()
        .skip(1)
        .map(|v| v.parse::<f32>().expect("Number only"))
        .collect();

    let pts = point(vec);
    println!("Points: {:?}", pts);

    // No 1.1
    let mut ascending_order = pts.clone();
    let mut descending_order = pts.clone();
    ascending_order.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    descending_order.sort_by(|(x1, y1), (x0, y0)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
    println!("Ascending: {:?}", ascending_order);
    println!("Descending: {:?}", descending_order);

    println!();

    // No 1.2
    let (ascending_order_b, descending_order_b) = bubble_sort(pts.clone());
    println!("Ascending with Bubble sort: {:?}", ascending_order_b);
    println!("Descending with Bubble sort: {:?}", descending_order_b);
}

#[test]
fn test_empty_input() {
    let pts = vec![];
    let (ascending, descending) = bubble_sort(pts);

    assert!(ascending.is_empty());
    assert!(descending.is_empty());

    let result = point(vec![]);
    assert!(result.is_empty());

    let empty_pts: Vec<(f32, f32)> = vec![];
    let mut empty_pts_clone = empty_pts.clone();

    empty_pts_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(empty_pts_clone.is_empty());

    empty_pts_clone.sort_by(|a, b| b.partial_cmp(a).unwrap());
    assert!(empty_pts_clone.is_empty());
}

#[test]
fn test_sort() {
    let vec = vec![1.0, 5.0, 2.0, 7.0, 3.0, 2.0, 4.0, 8.0];
    let pts = point(vec);

    //test points
    assert_eq!(pts, vec![(1.0, 5.0), (2.0, 7.0), (3.0, 2.0), (4.0, 8.0)]);

    let(ascending_b, descending_b) = bubble_sort(pts.clone());
    // Test ascending order bubble sort
    let expected_ascending_b = vec![(1.0, 5.0), (2.0, 7.0), (3.0, 2.0), (4.0, 8.0)];
    assert_eq!(ascending_b, expected_ascending_b);

    // Test descending order bubble sort
    let expected_descending_b = vec![(4.0, 8.0), (3.0, 2.0), (2.0, 7.0), (1.0, 5.0)];
    assert_eq!(descending_b, expected_descending_b);

     // Ascending order vec::sort
     let mut ascending = pts.clone();
     ascending.sort_by(|(x0, y0), (x1, y1)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
 
     let expected_ascending = vec![(1.0, 5.0), (2.0, 7.0), (3.0, 2.0), (4.0, 8.0)];
     assert_eq!(ascending, expected_ascending);
 
     // Descending order vec::sort
     let mut descending = pts.clone();
     descending.sort_by(|(x1, y1), (x0, y0)| x0.partial_cmp(x1).unwrap().then(y0.partial_cmp(y1).unwrap()));
 
     let expected_descending = vec![(4.0, 8.0), (3.0, 2.0), (2.0, 7.0), (1.0, 5.0)];
     assert_eq!(descending, expected_descending);
}