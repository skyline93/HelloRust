fn main() {
    // let number = 0;

    // if number > 5 {
    //     println!("condition was true");
    // } else if number == 0 {
    //     println!("number is zero");
    // }
    // else {
    //     println!("condition was false");
    // }

    // if number != 0 {
    //     println!("number is not zero");
    // }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);

    // let condition = true;
    // let number = if condition { 5 } else {"abc"};   // if表达式中返回值类型不一样，编译将报错
    // println!("number: {}", number);
}
