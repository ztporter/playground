const XMAS_PRESENTS: [&str; 12] = [ "  a partridge in a pear tree",
                                    "  two turtle doves",
                                    "  three french hens",
                                    "  four calling birds",
                                    "  five golden rings",
                                    "  six geese a-laying",
                                    "  seven swans a-swimming",
                                    "  eight maids a-milking",
                                    "  nine ladies dancing",
                                    "  ten lords a-leaping",
                                    "  eleven pipers piping",
                                    "  twelve drummers drumming" ];
const DAY_SUFFIX: [&str; 12] = [ "st", "nd", "rd", "th",
                                 "th", "th", "th", "th",
                                 "th", "th", "th", "th"];

fn main() {
    for number in (1..13) {
        print_day_of_xmas(number);
    }
}

fn print_day_of_xmas(day: usize) {
    println!("");
    println!("On the {}{} day of xmas, my true love gave to me...", day, DAY_SUFFIX[day - 1]);
    print_xmas_gifts(day);
}

fn print_xmas_gifts(day: usize) {
    println!("{}", XMAS_PRESENTS[day - 1]);
    if day > 1 {
        print_xmas_gifts(day - 1);
    }
}
