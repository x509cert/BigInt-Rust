use num_primes::Generator;
use num_bigint::BigUint;
use std::time::Instant;

const BITS: usize = 2048;

fn count_digits(n: &BigUint) -> usize {
  n.to_str_radix(10).len()
}

// we need this because there is num_bigint::BigUint and num_primes::BigUint
// and they are not the same type, even though they are the same under the covers
fn convert_to_bigint_from_num_bigint(n: num_primes::BigUint) -> BigUint {
  let bytes = n.to_bytes_be();
  BigUint::from_bytes_be(&bytes)
}

fn main() {

  let start = Instant::now();

  // Generate two primes (p,q) of n-bits each
  println!("Generating large primes, p & q, please be patient.");
  println!("Generating p.");
  let p = convert_to_bigint_from_num_bigint(Generator::new_prime(BITS));
  println!("Generating q.");
  let q = convert_to_bigint_from_num_bigint(Generator::new_prime(BITS));
  
  // Multiply to get the modulus (n)
  println!("Multiplying p and q.");
  let n = &p * &q;
  println!("p: {} digits, {}", count_digits(&p), p);
  println!("q: {} digits, {}", count_digits(&q), q);
  println!("n (p * q): {} digits, {}", count_digits(&n), n);

  let duration = start.elapsed();
  if cfg!(debug_assertions) {
    println!("Debug build");
  } else {
    println!("Release build");
  }
  println!("Time elapsed is: {:?}", duration);
}