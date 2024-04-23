fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut prefix = strs[0].to_string();
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}

fn main() {
    let strs = vec!["flower","flow","flight"];
    let lcp = longest_common_prefix(strs);
    println!("The longest common prefix is: {}", lcp);
}
