use std::mem::swap;
use std::time::Instant;

use rand::{Rng, RngExt};

fn main() {
    let mut numbers: Vec<i32> = generate_random_numbers(-10000, 10000, 20000);
    let mut numbers2 = numbers.clone();
    let mut numbers3 = numbers.clone();

    //println!("{:?}", numbers);
    bubble_sort(&mut numbers);
    selection_sort(&mut numbers2);
    insertion_sort(&mut numbers3);
}

fn bubble_sort(numbers: &mut Vec<i32>) {
    let mut sorted = false;
    let end_index = numbers.len() - 1;
    let mut comp_cnt = 0;
    let mut swap_cnt = 0;

    println!("##### Running Bubble Sort #####");
    println!("List size: {}", numbers.len());
    println!("Ending index: {}", end_index-1);

    let start = Instant::now();
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
    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);
    println!("Compares: {}", comp_cnt);
    println!("Swaps: {}", swap_cnt);
    //println!("{:?}", numbers);
}

fn insertion_sort(numbers: &mut Vec<i32>) {
    let mut comp_cnt = 0;
    let mut shift_cnt = 0;

    println!("##### Running Insertion Sort #####");
    println!("List size: {}", numbers.len());
    println!("Ending index: {}", numbers.len()-1);

    let start = Instant::now();
    for i in 1..numbers.len() {
        let mut index = i;
        let tmp = numbers[index];

        while index >= 1 {
            comp_cnt += 1;
            if tmp < numbers[index-1] {
                numbers[index] = numbers[index-1]; //Shift right
                shift_cnt += 1;
            }
            else {
                break;
            }
            index -= 1;
        }
        numbers[index] = tmp;
    }
    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);
    println!("Compares: {}", comp_cnt);
    println!("Shifts: {}", shift_cnt);
    //println!("{:?}", numbers);
}

fn selection_sort(numbers: &mut Vec<i32>) {
    let mut comp_cnt = 0;
    let mut swap_cnt = 0;
    let end_index = numbers.len() - 1;

    println!("##### Running Selection Sort #####");
    println!("List size: {}", numbers.len());
    println!("Ending index: {}", end_index-1);

    let start = Instant::now();
    for i in 0..end_index {
        let mut index = i;
        for i2 in (i+1)..numbers.len() {
            if numbers[i2] < numbers[index] {
                index = i2;
            }
            comp_cnt += 1;
        }
        if index > i {
            numbers.swap(i, index);
            swap_cnt += 1;
            }
    }
    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);
    println!("Compares: {}", comp_cnt);
    println!("Swaps: {}", swap_cnt);
    //println!("{:?}", numbers);
}

fn generate_random_numbers(min: i32, max: i32, count: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut numbers = Vec::new();

    for _ in 0..count {
        numbers.push(rng.random_range(min..=max));
    }

    numbers
}