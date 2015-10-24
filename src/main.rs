use std::f64::consts::PI;
use std::vec::Vec;

fn main() {
    let size = 4096;
    println!("Size: {}", size);

    let multiplier = 4096 * 64 * 8;
    println!("Multiplier: {}", multiplier);

    let step_size = 0.1;
    println!("Step size: {}", step_size);

    let sigma = find_sigma(size, 1.0 / (multiplier as f64), step_size);
    println!("Sigma: {}", sigma);

    let kernel = generate_kernel(size, sigma);
    /*println!("Kernel:");
    for weight in kernel.iter() {
        println!("{}", weight);
    }*/

    let integer_kernel = kernel.iter().map(|x| (x * (multiplier as f64)) as i32).collect::<Vec<_>>();
    /*println!("Integer kernel:");
    for weight in integer_kernel.iter() {
        println!("{}", weight);
    }*/

    println!("Pretty half integer kernel:");
    let row_length = 16;
    for i in 0..(size / 2 / row_length) {
        print!("    ");
        for j in 0..row_length {
            print!("{: >4},", integer_kernel[i * row_length + j]);
            if j < row_length - 1 {
                print!(" ");
            } else {
                println!("");
            }
        }
    }

    let sum = kernel.iter().fold(0.0, |sum, x| sum + x);
    println!("Sum: {}", sum);

    let integer_sum = integer_kernel.iter().fold(0, |sum, x| sum + x);
    println!("Integer sum: {}", integer_sum);

    let average_sum =
        (0..size)
        .fold(0, |acc, x| {
            let sum =
                integer_kernel[x % size] +
                integer_kernel[(x + size / 4) % size] +
                integer_kernel[(x + size / 2) % size] +
                integer_kernel[(x + size * 3 / 4) % size];
            acc + sum
        }) / (size as i32);
    println!("Average sum: {}", average_sum);

    println!("{}", 1 << 11);
}

fn find_sigma(size: usize, threshold: f64, step_size: f64) -> f64 {
    let mut current_sigma = step_size;
    loop {
        let kernel = generate_kernel(size, current_sigma);
        if kernel[size - 1] >= threshold {
            return current_sigma - step_size;
        }

        current_sigma += step_size;
    }
}

fn generate_kernel(size: usize, sigma: f64) -> Vec<f64> {
    (0..size).map(|x| gaussian(sigma, (((x as i32) - ((size as i32 / 2))) as f64))).collect::<Vec<_>>()
}

fn gaussian(sigma: f64, x: f64) -> f64 {
    (1.0 / ((2.0 * PI).sqrt() * sigma)) * (-((x * x) / (2.0 * sigma * sigma))).exp()
}
