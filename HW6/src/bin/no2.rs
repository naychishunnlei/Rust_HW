fn pack_number_tuples3(l1: &[i32], l2: &[i32], l3: &[i32]) -> Vec<(i32, i32, i32)> {
    let max_len = l1.len().max(l2.len()).max(l3.len());
    let mut result = Vec::new();

    for i in 0..max_len {
        let a1 = l1.get(i).cloned().unwrap_or(0);
        let a2 = l2.get(i).cloned().unwrap_or(0);
        let a3 = l3.get(i).cloned().unwrap_or(0);
        result.push((a1, a2, a3));
    }

    result
}

fn pack_number_tuples_s3(l1: &[i32], l2: &[i32], l3: &[i32]) -> Vec<(i32, i32, i32)> {
    let min_len = l1.len().min(l2.len()).min(l3.len());
    let mut result = Vec::new();

    for i in 0..min_len {
        let a1 = l1[i];
        let a2 = l2[i];
        let a3 = l3[i];
        result.push((a1, a2, a3));
    }

    result
}

fn main() {
    let v1 = [1, 2];
    let v2 = [4, 3];
    let v3 = [5];
    let result1 = pack_number_tuples3(&v1, &v2, &v3);
    println!("{:?}", result1);
    let result2 = pack_number_tuples_s3(&v1, &v2, &v3);
    println!("{:?}", result2);
}

#[test]
fn test_pack_number_tuples3() {
    let (v1, v2, v3) = ([1, 2], [4, 3], [5]);
    let result = pack_number_tuples3(&v1, &v2, &v3);
    assert_eq!(result, vec![(1, 4, 5), (2, 3, 0)]);
}

#[test]
fn test_pack_number_tuples_s3() {
    let (v1, v2, v3) = ([1, 2], [4, 3], [5]);
    let result = pack_number_tuples_s3(&v1, &v2, &v3);
    assert_eq!(result, vec![(1, 4, 5)]);
}

#[test]
fn test_pack_number_tuples3_empty_input() {
    let (v1, v2, v3): ([i32; 0], [i32; 0], [i32; 0]) = ([], [], []);
    let result = pack_number_tuples3(&v1, &v2, &v3);
    assert_eq!(result, vec![]);
}

#[test]
fn test_pack_number_tuples_s3_empty_input() {
    let (v1, v2, v3): ([i32; 0], [i32; 0], [i32; 0]) = ([], [], []);
    let result = pack_number_tuples_s3(&v1, &v2, &v3);
    assert_eq!(result, vec![]);
}
