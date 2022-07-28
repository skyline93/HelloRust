#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// fn build_user(email: String, username: String) -> User {
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

fn main() {
    // let email = String::from("greene9832@163.com");
    // let username = String::from("greene");

    // let user = build_user(email, username);

    // println!("{}", user.username);
    // println!("{}", user.email);
    // println!("{}", user.active);
    // println!("{}", user.sign_in_count);

    let user1 = User {
        username: String::from("greene"),
        email: String::from("Glf9832@163.com"),
        active: true,
        sign_in_count: 1,
    };

    // user1.email = String::from("greene9832@163.com");

    println!("{:#?}", user1);
    // println!("{}", user1.username);
    // println!("{}", user1.email);
    // println!("{}", user1.active);
    // println!("{}", user1.sign_in_count);

    // let mut user2 = User{
    //     username: String::from("glf"),
    //     email: String::from("Glf9832@163.com"),
    //     ..user1
    // };

    // user2.email = String::from("greene9832@163.com");

    // println!("{}", user2.username);
    // println!("{}", user2.email);
    // println!("{}", user2.active);
    // println!("{}", user2.sign_in_count);

    // struct Color(i32, i32, i32);

    // let black = Color(2, 254, 689);

    // let (col1, col2, col3) = black;

    // let User { username: i, email: j, active: m, sign_in_count: n } = user1;

    // println!("{}", black.0);
    // println!("{}", black.1);
    // println!("{}", black.2);

    // println!("{}", i);
    // println!("{}", j);
    // println!("{}", m);
    // println!("{}", n);
}
