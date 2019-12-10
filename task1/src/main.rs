use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let fuel_sum: i64 = stdin_lock.lines().map(|line| line.unwrap().parse::<i64>()).map(|maybe_num| fuel(maybe_num.unwrap())).sum();
    println!("{}", fuel_sum);
}

fn fuel(mass: i64) -> i64 {
    let module_fuel = mass / 3 - 2;
    if module_fuel <= 0 {
        0
    } else {
        module_fuel + fuel(module_fuel)
    }
}
