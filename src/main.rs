use std::collections::HashMap;

fn main() {
    let vector: Vec<i32> = vec![23, 12, 7, 23, 11, 23, 7, 4, 6, 2, 1, 76, 43, 52, 12, 65];

    let mean = mean(&vector);

    println!("Vector in question: {:?}", vector);

    println!("Mean is {}", mean);

    let (mode, freq) = mode(&vector);

    println!("Mode is {} and it occurs {} times", mode, freq);
}

fn mean(vector: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut length = 0;

    for i in vector {
        sum += i;
        length += 1
    }

    sum / length
}

fn mode(vector: &Vec<i32>) -> (i32, i32) {
    let mut modes: HashMap<i32, i32> = HashMap::new();

    for i in vector {
        let val = modes.entry(*i).or_insert(0);
        *val += 1;
    }

    let mut max = 0;
    let mut max_key : i32 = 0;

    for (key, val) in modes {
        if val > max {
            max_key = key;
            max = val
        }
    }

    (max_key, max)
}