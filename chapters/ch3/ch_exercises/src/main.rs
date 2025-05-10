mod exercise1;
mod exercise2;
use exercise1::convert_temps;
use exercise2::get_numbers;

fn main() {
    // Exercise 1
    let temps: [f64; 5] = [32.0, 10.0, 78.0, 99.0, 0.0];
    convert_temps(temps);

    // Exercise 2
    let arr_n: [u64; 10] = [1, 2, 3, 4, 5, 10, 20, 30, 40, 50];
    get_numbers(arr_n);
}
