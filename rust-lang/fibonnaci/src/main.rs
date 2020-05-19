use std::io;

fn main() {
    println!("print nth fibonnaci term: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("input failure");

    let nterm: u32 = input.trim().parse()
        .expect("please enter a natural number!");

    println!("{}th fibonacci term is {}", nterm, fib(nterm));
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
