// main.rs

fn main() {
    let a = vec![0, 1, 2];

    let mut i = 0;
    while i < 5 {
        println!("a[{}] = {}", i, a[i]);
        i += 1;
    }
}

/* Handle an error using a match */

// fn main() {
//     let a = vec![0, 1, 2];

//     let mut i = 0;
//     while i < 5 {
//         match a.get(i) {
//             Some(val) => {
//                 println!("a[{}] = {}", i, val);
//             }
//             None => {
//                 println!("{} is out of bounds", i);
//             }
//         }
//         i += 1;
//     }
// }

/* More Idiomatic code */

// fn main() {
//     let a = vec![0, 1, 2];
//     for (i, val) in a.iter().enumerate() {
//         println!("a[{}] = {}", i, val)
//     }
// }
