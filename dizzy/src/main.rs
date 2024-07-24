#![deny(clippy::all)]

fn main() {
    let mut counter = 0;
    let mut number = 2;
    let result = loop {
        let mut prime_number = 0;
        let mut is_prime: bool = true;

        for num in 2..number {
            if is_prime {
                if number % num == 0 {
                    is_prime = false;
                }
            } else {
                break;
            }
        }
        // if is true
        if is_prime {
            counter += 1;
            prime_number = number;
            println!("counter : {:#?} , number is : {:#?}", counter, prime_number)
        }

        if counter == 1001 {
            break prime_number;
        }
        number += 1;
    };
    println!(" result = {:?}", result);
}
