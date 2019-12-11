use std::cmp::Ordering;

fn main() {
    let valid_passwords: Vec<u32> = (145852..616943).into_iter().filter(|x| check_number(*x as u32)).collect();
    println!("{}, {:?}", valid_passwords.len(), valid_passwords);
}

fn check_number(x: u32) -> bool {
    let digits: Vec<u32> = (0..6).map(|i| digit(x, i)).collect();
    let mut double_found = false;
    let mut in_a_run = false;
    let mut run_length = 0;
    let mut run_digit = 10;
    for i in 0..5 {
        let comparison = digits[i].cmp(&digits[i+1]);
        match comparison {
            Ordering::Less => return false,
            Ordering::Equal => {
                in_a_run = true;
                if run_digit == digits[i] {
                    run_length += 1;
                } else {
                    run_digit = digits[i];
                    run_length = 2;
                }
            },
            Ordering::Greater => {
                if in_a_run && run_length == 2 {
                    double_found = true;
                }
                in_a_run = false;
                run_length = 0;
                run_digit = 10;
            }
        };
    }
    if in_a_run && run_length == 2 {
        double_found = true;
    }
    double_found
}

fn digit(x: u32, i: u32) -> u32 {
    let ten = 10u32.pow(i);
    (x % (ten * 10)) / ten
}
