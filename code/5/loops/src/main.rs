fn main() {
    // loop {
    //     println!("again!");
    // }

    // ======loop===================================
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;      // 由于break是最后一个表达式，这里的分号可有可无
    //     }
    // };

    // println!("counter: {}", result)

    // =======while==================================
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number = number - 1;
    // }

    // println!("LIFTOFF!!!");

    // ========for====================================
    // let a = [10, 20, 30, 40, 50, 60];

    // for i in a.iter() {
    //     println!("the value is {}", i);
    // }

    // =============================================
    for number in (1..4).rev() {    // rev用来翻转序列
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
