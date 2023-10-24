use num_primes::Generator;
use num_bigint::BigUint;

const BITS: usize = 1024;

fn count_digits(n: &BigUint) -> usize {
  n.to_str_radix(10).len()
}

// we need this because there is num_bigint::BigUint and num_primes::BigUint
// and they are not the same
fn convert_to_bigint_from_num_bigint(n: num_primes::BigUint) -> BigUint {
  BigUint::parse_bytes(n.to_str_radix(10).as_bytes(), 10).unwrap()
}

fn main(){
  // Generate two primes (p,q) of n-bits each
  println!("Generating large primes, p & q, please be patient.");
  let p = convert_to_bigint_from_num_bigint(Generator::new_prime(BITS));
  let q = convert_to_bigint_from_num_bigint(Generator::new_prime(BITS));
  
  // Multiply to get the modulus (n)
  let n = &p * &q;
  println!("p: {} digits, {}", count_digits(&p), p);
  println!("q: {} digits, {}", count_digits(&q), q);
  println!("n (p * q): {} digits, {}", count_digits(&n), n);
}