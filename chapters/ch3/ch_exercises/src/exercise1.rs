pub fn convert_temps(temps: [f64; 5]) {
    for t in temps {
        let celsius = temp_converter(t, true);
        let fahrenheit = temp_converter(celsius, false);
        println!("Starting F temp is: {t}");
        println!("Temp in C is: {celsius}");
        println!("Temp back in F is: {fahrenheit}\n");
    }
}

fn temp_converter(temp: f64, is_fahrenheit: bool) -> f64 {
    if !is_fahrenheit {
        return 1.8 * temp + 32.0;
    } else {
        return (temp - 32.0) / 1.8;
    }
}
