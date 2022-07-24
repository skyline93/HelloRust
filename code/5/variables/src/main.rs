fn main() {
    // 4.1.1、不可变性
    // let x = 5;
    // println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);

    // 4.1.2、可变性
    // let mut x = 5;                              // 变量前增加mut关键字使变量具有可变性
    // println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);

    // 4.1.3、隐藏
    // let x = 5;
    // println!("The value of x is {}", x);
    // let x = 6;                               // 将x隐藏，x的值为6
    // println!("The value of x is {}", x);

    // let x = "abc";  // 变量x是字符串类型
    // println!("The value of x is {}", x);
    // let x = 1;       // 变量x变成了数值
    // println!("The value of x is {}", x);

    // let mut x = "abc";  // 变量x是字符串类型
    // x = 1;               // 变量x变成了数值

    // 4.2 常量
    // const MAX_POINTS: u32 = 100_000;

    // let x = 1;
    // let x = x * MAX_POINTS;
    // println!("The value of x is {}", x);
    // println!("The const value of x is {}", MAX_POINTS);

    // let y = 2;
    // MAX_POINTS = y;     // 无法将运行时的值绑定到常量上，这里会报错

    // 5.3.1 整数溢出
    // let mut i: u8 = 255;
    // println!("i: {}", i);

    // i = 256;
    // println!("i: {}",i);

    // 5.3.3 数值运算
    // let sum = 5 + 10;
    // println!("sum: {}", sum);

    // let difference = 95.5 - 4.3;
    // println!("difference: {}", difference);

    // let product = 4 * 50;
    // println!("product: {}", product);

    // let quotient = 56.7 / 32.2; // 除法
    // println!("quotient: {}", quotient);

    // let remainder = 43 % 5; // 取余
    // println!("remainder: {}", remainder);

    // let a: u32 = 10;
    // let b: u64 = 1357;

    // let c = a + b;  // 不同类型的数值不能做运算
    // println!("c: {}", c);

    // 5.3.5、字符类型
    // let c = 'x';
    // println!("c: {}", c);

    // let emoji = '😻';
    // println!("emoji: {}", emoji);

    // 5.3.6、元组类型
    // let t: (u32, f64, u8, char);
    // println!("t: {}, {}, {}, {}", t.0, t.1, t.2, t.3);  // 使用点号访问索引下标

    // let t = (500, 6.4, 2, 'c');
    // println!("t: {}, {}, {}, {}", t.0, t.1, t.2, t.3);  // 使用点号访问索引下标

    // let t: (u32, f64, u8, char) = (500, 6.4, 2, 'c');
    // println!("t: {}, {}, {}, {}", t.0, t.1, t.2, t.3);

    // let (a, b, c, d) = t;       // 解构
    // println!("t: {}, {}, {}, {}", a, b, c, d);

    // // t.0 = 200;  // t是不可变的
    // // println!("t: {}, {}, {}, {}", t.0, t.1, t.2, t.3);

    // let mut x = (500, 6.4, 2, 'c');
    // println!("x: {}, {}, {}, {}", x.0, x.1, x.2, x.3);

    // x.0 = 200;
    // println!("x: {}, {}, {}, {}", x.0, x.1, x.2, x.3);

    // // x.3 = 5; // 不同类型不允许赋不同类型的值
    // // println!("x: {}, {}, {}, {}", x.0, x.1, x.2, x.3);

    // 5.3.7、数组类型
    let a = [1, 2, 3, 4, 5];
    println!("a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]); // 使用方括号访问索引下标

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);

    let a = [3; 5];
    println!("a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);

    let a = ['b'; 5];
    println!("a: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);

    let b = [1,2,3,4,5];
    let index = 8;
    let element = b[index]; // 这里索引越界了，但是cargo check时不会报错，在运行时才发生panic使程序崩溃
    println!("element: {}", element);
}
