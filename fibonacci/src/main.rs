fn main() {
    let n = 10;

    let mut number = 1;
    let mut number_last = 1;

    for _i in 1..(n-1) {
        let tmp = number;
        number = number + number_last;
        number_last = tmp;
    }

    println!("The {n}th Fibonacci number is {number}");
}
