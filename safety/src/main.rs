// main.rs

/*** Move ***/

// fn main() {
//     let words = vec!["Rust:", "safe", "fast", "productive"];
//     list(words);
//     // list(words);
// }

// fn list(words: Vec<&str>) {
//     for word in words {
//         print!("{}\n", word);
//     }
// }

/*** Borrow ***/

// fn main() {
//     let words = vec!["Rust:", "safe", "fast", "productive"];
//     list(&words);
//     list(&words);
// }

// fn list(words: &Vec<&str>) {
//     for word in words {
//         print!("{}\n", word);
//     }
// }

/*** Mutable Borrow ***/

fn main() {
    let mut words = vec!["Rust:", "safe", "fast", "productive"];
    list(&words);
    and_fun(&mut words);
    list(&words);
}

fn list(words: &Vec<&str>) {
    for word in words {
        print!("{}\n", word);
    }
}

fn and_fun(words: &mut Vec<&str>) {
    let s = "fun";
    words.push(&s);
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
