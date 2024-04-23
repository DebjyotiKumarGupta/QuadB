fn merge_sorted_arrays(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut merged = vec![];
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }

    while i < a.len() {
        merged.push(a[i]);
        i += 1;
    }

    while j < b.len() {
        merged.push(b[j]);
        j += 1;
    }

    merged
}

fn main() {
    let a = vec![1, 3, 5];
    let b = vec![2, 4, 6];
    let merged = merge_sorted_arrays(a, b);
    println!("Merged array: {:?}", merged);
}
