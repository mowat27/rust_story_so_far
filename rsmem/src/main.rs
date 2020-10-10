// main.rs

fn main() {
    let a = vec![0, 1, 2];

    let mut i = 0;
    while i < 5 {
        println!("a[{}] = {}", i, a[i]);
        i += 1;
    }
}
