fn main() {
    let mut string1 = String::from("hello, ");
    let string2 = "world!";
    string1.push_str(string2);
    println!("{}", string1);
}
