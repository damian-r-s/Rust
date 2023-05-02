use std::env::{args, Args};

fn main() {
    let mut args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = operateMatching(operator, first_number, second_number);

    println!(
        "{:?}",
        output(first_number, operator, second_number, result)
    );
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    if operator == '+' {
        first_number + second_number
    } else if operator == '-' {
        first_number - second_number
    } else if operator == '/' {
        first_number / second_number
    } else if operator == '*' {
        first_number * second_number
    } else {
        0.0
    }
}

fn operateMatching(operator: char, first: f32, second: f32) -> f32 {
    match operator {
        '+' => first + second,
        '-' => first - second,
        '/' => first / second,
        '*' => first * second,
        _ => panic!("Invalid operator used!"),
    }
}

fn output(first: f32, operator: char, second: f32, result: f32) -> String {
    format!("{} {} {} = {}", first, operator, second, result)
}
