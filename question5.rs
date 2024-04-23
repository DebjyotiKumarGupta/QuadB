fn median(arr: &[i32]) -> f32 {
    let len = arr.len();
    let mid = len / 2;

    if len % 2 == 0 {
        (arr[mid - 1] + arr[mid]) as f32 / 2.0
    } else {
        arr[mid] as f32
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let med = median(&arr);
    println!("The median is: {}", med);
}
