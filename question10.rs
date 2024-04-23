fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _even if n % 2 == 0 => false,
        _ => !(3..).step_by(2).take_while(|i| i * i <= n).any(|i| n % i == 0),
    }
}

fn main() {
    let num = 7;
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
