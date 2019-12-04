use std::env;

fn check_armstrong(num: i64) -> bool {
    use std::convert::TryInto;
    let mut sum: i64 = 0;

    let mut digits: std::vec::Vec<i64> = std::vec::Vec::new();
    for digit in num.to_string().chars() {
        digits.push(digit.to_string().parse().unwrap());
    }

    for i in 0..digits.len().try_into().unwrap() {
        sum += digits[i].pow(digits.len().try_into().unwrap());
    }

    let is_armstrong: bool = num == sum;

    print!("{} is {} Armstrong number, ", num, if is_armstrong { "an" } else { "not an" });
    print!("because {} {} ", num, if is_armstrong { "=" } else { "!=" });

    let num_length: i64 = digits.len().try_into().unwrap();

    for i in 0..digits.len().try_into().unwrap() {
        print!("{}^{}{}", digits[i], num_length, if i == (num_length - 1).try_into().unwrap() { " " } else { " + " });
    }

    println!("= {}", sum);
    is_armstrong
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().count() >= 2 {
        let num: i64 = args[1].parse().expect("Not a valid number!");
        let _is_armstrong: bool = check_armstrong(num);
    } else {
        println!("No number was entered.");
    }
}
