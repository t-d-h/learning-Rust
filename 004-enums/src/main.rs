// /3 -> fizz
// /5 -> buzz
// both -? fizzbuzz

enum FizzBuzzValue {
    Fizz, // a variant
    Buzz,
    FizzBuzz,
    NotDivisible(i32),
}

fn main() {
    for i in 1..100 {
        print_fizzbuzz(i);
    }
}

fn print_fizzbuzz(x: i32) { //the presentation of results to user
    match fizzbuzz(x) {
        FizzBuzzValue::FizzBuzz => {
            println!("Fizzbuzz");
        }
        FizzBuzzValue::Fizz => {
            println!("Fizz");
        }
        FizzBuzzValue::Buzz => {
            println!("Buzz");
        }
        FizzBuzzValue::NotDivisible(num) => {
            println!("{}", num);
        }
    }
}

fn fizzbuzz(x: i32) -> FizzBuzzValue { //the computation of results
    if x%3 == 0 && x%5 == 0 {
        FizzBuzzValue::FizzBuzz
    } else if x%3 == 0 {
        FizzBuzzValue::Fizz
    } else if x%5 == 0 {
        FizzBuzzValue::Buzz
    } else {
        FizzBuzzValue::NotDivisible(x)
    }
}

