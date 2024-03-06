fn main() {
    let member = 2_usize.pow(25) + 1;

    let mut n1: i32 = 1;
    let mut n2: i32 = 2;
    let mut n3: i32 = 3;

    let mut n = 3;

    while n < member
    {
        let new: i32 = n1 - 2 * &n2 + 3 * &n3;
        n1 = n2;
        n2 = n3;
        n3 = new % 1000000;
        n += 1;
    }
    println!("{0}", n3);
}