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
    const MAX_POINTS: u32 = 100_000;

    let x = 1;
    let x = x * MAX_POINTS;
    println!("The value of x is {}", x);
    println!("The const value of x is {}", MAX_POINTS);

    // let y = 2;
    // MAX_POINTS = y;     // 无法将运行时的值绑定到常量上，这里会报错
}
