fn min_max_avg(v: &[i32]) -> (i32, i32, f32) {
    let mut sum = 0.0;
    let mut iter = v.iter();
    if let Some(mut min) = iter.next() {
        let mut max = *min;
        while let Some(x) = iter.next() {
            if x < min {
                min = x;
            }
            if x > &max {
                max = *x;
            }
            sum += *x as f32;
        }
        let len = v.len() as f32;
        let avg = sum/len;
        return (*min, max, avg);

    } else {
        (0, 0, 0.)
    }
}

fn cal_partial_sums(v:&[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for _i in 0..v.len() {
        if _i==0 {
            result.push(v[_i]);
        }
        else {
            let res = v[_i]+result[_i-1];
            result.push(res);
        }
    }
    result
}

fn main() {
    let v = vec![2, 11, 3, 4, 7];

    let result1 = min_max_avg(&v);
    let result2 = cal_partial_sums(&v);

    println!("{:?}", result1);
    println!("{:?}", result2);
}

#[test]
fn test_min_max_avg_with_empty() {
    let v: [i32; 0] = [];
    let result = (0, 0, 0.);
    assert_eq!(min_max_avg(&v), result);
}

#[test]
fn test_cal_partial_sums_with_empty() {
    let v: [i32; 0] = [];
    let result = Vec::new();
    assert_eq!(cal_partial_sums(&v), result);
}

#[test]
fn test_with_values() {
    let v = vec![2, 11, 3, 4, 7];
    let result1 = (2, 11, 5.0);
    let result2 = vec![2, 13, 16, 20, 27];

    assert_eq!(min_max_avg(&v), result1);
    assert_eq!(cal_partial_sums(&v), result2);
}