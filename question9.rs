fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "Hello, world!";
    let reversed = reverse_string(s);
    println!("The reversed string is: {}", reversed);
}
