use ndarray::{Array2, ArrayView2, s};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use std::time::{Duration, Instant};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: cargo run <rows> <cols> <verbose>");
        std::process::exit(1);
    }

    let rows: usize = args[1].parse().expect("rows must be an integer");
    let cols: usize = args[2].parse().expect("cols must be an integer");
    let verbose: bool = args[3].parse().expect("verbose must be boolean");

    // Matrix M creation
    let m = Array2::<u8>::random((rows, cols), Uniform::new(0, 255));

    if verbose {
        println!("Matrix M:\n{}", m);
    }

    // Convolution along the horizontal axis
    let start_time_dx = Instant::now();
    let dx = convolve_horizontal(&m);
    let duration_dx = start_time_dx.elapsed();

    if verbose {
        println!("Matrix Dx:\n{}", dx);
    }

    // Display timing for Dx
    println!("Time for Dx: {} microseconds", duration_dx.as_nanos());

    // Convolution along the vertical axis
    let start_time_dy = Instant::now();
    let dy = convolve_vertical(&m);
    let duration_dy = start_time_dy.elapsed();

    if verbose {
        println!("Matrix Dy:\n{}", dy);
    }

    // Display timing for Dy
    println!("Time for Dy: {} microseconds", duration_dy.as_nanos());
}

fn convolve_horizontal(m: &Array2<u8>) -> Array2<u8> {
    let kernel = [-1, 0, 1];
    let rows = m.nrows();
    let cols = m.ncols() - 2; // Adjust for kernel width
    let mut result = Array2::<u8>::zeros((rows, cols));

    for i in 0..rows {
        for j in 0..cols {
            let convolution = kernel.iter().enumerate().fold(0i32, |acc, (k, &weight)| {
                acc + weight * m[[i, j + k]] as i32
            });
            // Handle overflow and underflow
            result[[i, j]] = convolution.max(0).min(255) as u8;
        }
    }
    result
}

fn convolve_vertical(m: &Array2<u8>) -> Array2<u8> {
    let kernel = [-1, 0, 1];
    let rows = m.nrows() - 2; // Adjust for kernel height
    let cols = m.ncols();
    let mut result = Array2::<u8>::zeros((rows, cols));

    for i in 0..rows {
        for j in 0..cols {
            let convolution = kernel.iter().enumerate().fold(0i32, |acc, (k, &weight)| {
                acc + weight * m[[i + k, j]] as i32
            });
            // Handle overflow and underflow
            result[[i, j]] = convolution.max(0).min(255) as u8;
        }
    }
    result
}
