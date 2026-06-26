use rand::{Rng, RngExt};

fn main() {
    let mut numbers: Vec<i32> =generate_random_numbers(-10000, 10000, 100);

    println!("{:?}", numbers);
    bubble_sort(&mut numbers);
}

fn bubble_sort(numbers: &mut Vec<i32>) {
    let mut sorted = false;
    let end_index = numbers.len() - 1;
    let mut comp_cnt = 0;
    let mut swap_cnt = 0;

    println!("List size: {}", numbers.len());
    println!("Ending index: {}", end_index);

    while !sorted {
        sorted = true;
        
        for i in 0..end_index {
            comp_cnt += 1;
            if numbers[i] > numbers[i+1] {
                numbers.swap(i, i+1);
                sorted = false;
                swap_cnt += 1;
            }
        }
    }
    println!("Compares: {}", comp_cnt);
    println!("Swaps: {}", swap_cnt);
    println!("{:?}", numbers);
}

fn generate_random_numbers(min: i32, max: i32, count: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut numbers = Vec::new();

    for _ in 0..count {
        numbers.push(rng.random_range(min..=max));
    }

    numbers
}