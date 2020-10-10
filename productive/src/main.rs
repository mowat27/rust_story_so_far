// main.rs

fn main() {
    let x = 7;
    let y = 9;
    println!("{} + {} = {}", x, y, add(x, y));
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
