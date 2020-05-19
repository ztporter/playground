use std::collections::HashMap;

fn main() {
    let input_list = [98, 67, 88, 79, 85, 81, 94, 91, 85];

    let sorted_vec: Vec<i32> = list_to_sorted_vec(&input_list);

    println!("sorted vector: {:?}", &sorted_vec);
    println!("mean: {}", mean(&sorted_vec));
    println!("median: {}", median(&sorted_vec));
    println!("mode: {}", get_mode_from_vec(&sorted_vec).unwrap());
}

fn list_to_sorted_vec (list: &[i32]) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for &i in list.iter() {
        vec.push(i);
    }
    vec.sort();
    vec
}

fn mean (vec: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in vec {
        sum += i;
    }
    let sum: f64 = sum.into();
    let length: f64 = vec.len() as f64;
    sum / length
}

fn median (sorted_vec: &Vec<i32>) -> i32 {
    sorted_vec[sorted_vec.len() / 2]
}

fn get_mode_from_vec (vec: &Vec<i32>) -> Option<i32> {
    let map = hash_map_from_vec(vec);
    let mut max = 0;
    let mut mode: Option<i32> = Option::None;
    
    for (key, value) in map {
        if value > max {
            mode = Option::Some(key);
            max = value;
        }
    }
    
    mode
}

fn hash_map_from_vec (vec: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    for i in vec {
        let count: &mut i32 = map.entry(*i).or_insert(0);
        *count +=1;
    }
    map
}
