fn main() {

    let mut num1: i32 = 1;
    let mut num2: i32 = 2;

    let mut sum: i32 = 0;

    loop {
        let tmp: i32 = num2;
        num2 = num1 + num2;
        num1 = tmp;

        if num1 % 2 == 0 {
            sum += num1;
        }

        if num2 >= 4000000 {
            break;
        }
    }
    println!("{sum}");
}
