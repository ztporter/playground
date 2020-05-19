fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 53, 79, 64, 2];
    println!("Largest number is {}", largest(&number_list));

    let char_list = vec!['z', 'g', 'a', 'q'];
    println!("Largest char is {}", largest(&char_list));
}
