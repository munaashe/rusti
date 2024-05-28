fn main() {
    const HOURS_IN_A_DAY: i32 = 24;
    let x = 4;
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x);
    }

    let x = "salibonani";
    println!("x is: {}", x);
    println!("{}", HOURS_IN_A_DAY)
}
