extern crate primal;

use primal::Sieve;

fn sieve_test(sieve: &Sieve) {
    let suspect = 5273;
    println!("{} is prime: {}", suspect, sieve.is_prime(suspect));
    let not_a_prime = 1024;
    println!("{} is prime: {}", not_a_prime, sieve.is_prime(not_a_prime));
}

fn thousand_prime_number(sieve: &Sieve) {
    let mut prime_count = 0;
    let mut number_count = 0;
    while prime_count < 1000 {
        number_count += 1;
        let mut result = sieve.is_prime(number_count);
        while result == false {
            number_count += 1;
            result = sieve.is_prime(number_count);
        }
        prime_count += 1;
    }
    println!("1000th Prime number is {}", number_count);
}

fn factor(number: usize, sieve: &Sieve) {
    println!("Factors of {} are {:?}", number, sieve.factor(number));
}

// fn main() {
//     let sieve = Sieve::new(10000);
//     println!("Primal Experimentation!");
//     sieve_test(&sieve);
//     thousand_prime_number(&sieve);
//     factor(24, &sieve);
// }
