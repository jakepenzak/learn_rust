pub fn get_numbers(arr_n: [u64; 10]) {
    for n in arr_n {
        let val: u64 = nth_fibonacci_number(n);
        println!("The {n} fibonacci number is {val}")
    }
}

fn nth_fibonacci_number(n: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut c: u64 = a + b;
    if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    } else {
        for _ in 1..n {
            c = a + b;
            a = b;
            b = c;
        }
    }
    return c;
}
