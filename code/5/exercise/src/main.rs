fn main() {
    const TOTAL: usize = 47;

    let mut num = 0;
    let mut list = [0; TOTAL];

    while {
        num += 1;
        num < 47
    } {
        if num == 1 {
            list[num] = list[num - 0] + 1;
            continue;
        }

        list[num] = list[num - 1] + list[num - 2];
    }

    for i in list.iter() {
        println!("list: {}", i);
    }
}
