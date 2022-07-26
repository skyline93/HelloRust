fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("s: {}", s);

    // ===============
    // let s1 = String::from("hello");

    // takes_ownership(s1);
    // // println!("{}", s1);

    // let x = 5;

    // makes_copy(x);
    // println!("x: {}", x);

    // ================
    // let s1 = gives_ownership();
    // println!("s1:{}", s1);

    // let s2 = String::from("hello");

    // let s3 = takes_and_gives_back(s2);
    // println!("s3:{}", s3);

    //==================
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);

    // println!("s2: {}, len:{}", s2, len);

    //==================
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("s1: {}, len: {}", s1, len);

    //==================
    // let mut s = String::from("hello");
    // change(&mut s);

    // println!("s: {}", s);

    //=====================
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // // println!("r1: {}, s: {}", r1, &mut s);
    // // println!("r1: {}", r1);
    // println!("r2: {}", r2);

    //=====================
    // let mut s = String::from("hello");

    // let r1 = &s;
    // let r2 = &s;
    // println!("s: {}, r1: {}, r2: {}", s, r1, r2);

    // s.push_str(", world");
    // let r3 = &s;

    // println!("r3: {}", r3);

    //==========================
    let s = String::from("hello world");
    
    let hello = &s[..5];
    let world = &s[6..];

    println!("hello: {}", hello);
    println!("world: {}", world);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world!");
// }
