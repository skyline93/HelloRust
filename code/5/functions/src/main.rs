fn main() {
    println!("Hello, world!");

    another_function();

    func1(127, 1068986, 'w');

    func2();

    let x = func3();
    println!("x: {}", x);

    let (a, b) = func4();
    println!("a: {}, b: {}", a, b);
}

fn another_function() {
    println!("Another function.");
}

fn func1(x: i8, y: u64, a: char) {
    println!("x, y, a, b : {}, {}, {}", x, y, a);
}

fn func2() {
    let mut x = 5;
    println!("x: {}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    x = x + 1;
    println!("x: {}", x);
}

fn func3() -> u8 {
    255;
    254
}

fn func4() -> (u8, char) {
    return (127, 'c');
    return (126, 'd');
}
