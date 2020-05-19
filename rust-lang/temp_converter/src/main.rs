use std::io;

fn main() {
    let mut input = String::new();

    println!("temperature to convert: ");

    io::stdin().read_line(&mut input)
        .expect("Input failure");

    let temp: f64 = input.trim().parse()
        .expect("please enter a number!");

    println!("convert to f or c?: ");

    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Input failure");

    input = input.trim().to_string();

    if input == "f" {
        println!("converting {} to f: {}", temp, c_to_f(temp));
    } else if input == "c" {
        println!("converting {} to c: {}", temp, f_to_c(temp));
    }
    else {
        println!("invalid option!");
    }
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f64) ->f64 {
    (c * 1.8) + 32.0
}
