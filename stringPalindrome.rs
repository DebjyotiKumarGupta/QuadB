use std::io::{self, Write};

fn check_palindrome(s: &str) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let (mut start, mut end) = (0, s.len() - 1);

    while start < end {
        if s[start] != s[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }

    true
}

fn main() {
    let mut input = String::new();
    print!("Enter the string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if check_palindrome(input) {
        println!("Yes! The string is a palindrome");
    } else {
        println!("No! The string is not a palindrome");
    }
}
