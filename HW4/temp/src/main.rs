//ordinary loop
fn fahr_to_cel_v(fahrenheit_temps: Vec<f32>) -> Vec<f32> {
    let mut celsius_temps = Vec::new();

    for fahr_temp in &fahrenheit_temps {
        let celsius_temp = (fahr_temp - 32.0) * 5.0 / 9.0;
        let celsius_temp = (celsius_temp*100.0).round()/100.0;
        celsius_temps.push(celsius_temp);
    }

    celsius_temps
}

//recursion
fn fahr_to_cel_recursion(v: &mut Vec<f32>, index: usize) {
    if index < v.len() {
        let celsius_temp = (5.0 / 9.0) * (v[index] - 32.0);
        v[index] = (celsius_temp * 100.0).round() / 100.0;
        fahr_to_cel_recursion(v, index + 1);
    }
}


#[test]
fn test_fahr_to_cel() {
    let fahr_temps = vec![10.0, 100.0, 200.0];
    let expected_cel_temps = vec![-12.22,37.78 ,93.33];
    let actual_cel_temps = fahr_to_cel_v(fahr_temps.clone());
    fahr_to_cel_recursion(&mut fahr_temps.clone(), 0);

    //test for ordinary loop
    assert_eq!(expected_cel_temps, actual_cel_temps);

    //test for recursion
    assert_eq!(expected_cel_temps, fahr_temps);
}

#[test]
fn test_fahr_to_cel_without_values() {
        let fahr_temps = vec![];
        let expected_cel_temps: Vec<f32> = vec![];
        let actual_cel_temps = fahr_to_cel_v(fahr_temps.clone());
        fahr_to_cel_recursion(&mut fahr_temps.clone(), 0);

        //test for ordinary loop
        assert_eq!(expected_cel_temps, actual_cel_temps);
    
        //test for recursion
        assert_eq!(expected_cel_temps, fahr_temps);
    }


fn main() {
    let fahr_temps = vec![32.0, 212.0, 50.0, -40.0];

    //using ordinary loop
    let cel_temps_loop = fahr_to_cel_v(fahr_temps.clone());
    println!("Using ordinary loop:");
    for (i, (fahr, cel)) in fahr_temps.iter().zip(cel_temps_loop.iter()).enumerate() {
        println!("Temp {}: {}°F = {}°C", i + 1, fahr, cel);
    }

    // Using recursion
    let mut cel_temps_recursion = fahr_temps.clone();
    fahr_to_cel_recursion(&mut cel_temps_recursion, 0);
    println!("Using recursion:");
    for (i, (fahr, cel)) in fahr_temps.iter().zip(cel_temps_recursion.iter()).enumerate() {
        println!("Temp {}: {}°F = {}°C", i + 1, fahr, cel);
    }
}