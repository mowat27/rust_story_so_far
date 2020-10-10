// main.rs

/*** Move ***/

// fn main() {
//     let words = vec!["Rust:", "safe", "fast", "productive"];
//     list(words);
//     list(words);
// }

// fn list(words: Vec<&str>) {
//     for word in words {
//         print!("{}\n", word);
//     }
// }

/*** Borrow ***/

fn main() {
    let words = vec!["Rust:", "safe", "fast", "productive"];
    list(&words);
    list(&words);
}

fn list(words: &Vec<&str>) {
    for word in words {
        print!("{}\n", word);
    }
}

/*** Move to Scope ***/

// fn main() {
//     let words = vec!["Rust:", "safe", "fast", "productive"];
//     {
//         for word in words {
//             print!("{}\n", word);
//         }
//     }
//     list(words);
// }

// fn list(words: Vec<&str>) {
//     for word in words {
//         print!("{}\n", word);
//     }
// }
