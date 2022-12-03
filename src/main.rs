/*
<----------------------------------
Author: Ilia Sichinava 
Date: December 3rd, 2022
Title: Linear Regression
---------------------------------->
*/
use core::panic;

// Define struct for unknown k and b values
struct Equation {
    k: f32,
    b: f32
}

fn main() {
    // Test data
    let data: Vec<(f32, f32)> = vec![(0.0, 2.10), (1.0, 1.92), (2.0, 1.84), (3.0, 1.71), (4.0, 1.64)];
    println!("Vertices data: {:?}", &data);

    // Linear regression prints the equation and returns k and b
    let eq = linear_regression(&data);

    // Test cases for different x values
    predict(&eq, 5.0);
    predict(&eq, 6.0);
    predict(&eq, 7.0);
    predict(&eq, 8.0);
    predict(&eq, 9.0);
}

// Returns k and b, prints equation
fn linear_regression(data: &Vec<(f32, f32)>) -> Equation {
    // Check for empty data
    if data.len() == 0 {
        panic!("Enter non-zero data")
    }

    // Find everything for our k and b
    let x_mean: f32 = data.iter().map(|point| point.0).sum::<f32>() / data.len() as f32;
    let y_mean: f32 = data.iter().map(|point| point.1).sum::<f32>() / data.len() as f32;
    let xy_mean: f32 = data.iter().map(|point| point.0 * point.1).sum::<f32>() / data.len() as f32;
    let x_square_mean: f32 = data.iter().map(|point| point.0 * point.0).sum::<f32>() / data.len() as f32;

    // ------------------------->
    if x_square_mean - (x_mean * x_mean) == 0.0 {
        panic!("Can't divide by zero")
    }

    // Now let's calculate them
    let k: f32 = (xy_mean - (x_mean * y_mean)) / (x_square_mean - (x_mean * x_mean));
    let b: f32 = y_mean - (k * x_mean);
    
    print!("Regression line: ");
    #[allow(illegal_floating_point_literal_pattern)]
    // Check for k and b to display data correctly. 
    // Print y = x + 3 instead of y = 1x + 3
    match (k, b) {
        // Cases where k = 1
        (1.0, 0.0) => println!("y = x"),
        (1.0, _) => println!("y = x + {:.3}", b),        
        (0.0, _) => println!("y = {:.3}", b),

        // Case where b = 0
        (_, 0.0) => println!("y = {:.3}x", k),
        
        // k != 1 && b != 0
        _ => println!("y = {:.3}x + {:.3}", k, b)
    }
    println!("-----------------------------------------");

    // Return found equation members and use them for predictions afterwards
    Equation { k, b }
}

fn predict(e: &Equation, x: f32) -> () {
    // Simply count y from y = kx + b equation
    println!("Predicted value for {:.3} is {:.3}", x, e.k * x + e.b)
}