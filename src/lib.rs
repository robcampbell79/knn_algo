use std::num::Float;

pub fn euclid_distance(point1: Vec<f64>, point2: f64) {
    let sum: f64 = 0;
    let sum_squared: f64 = 0;
    let i: f64 = 0;

    for i in 0..point1.len() {
        sum += point[i] - point2;
    }

    sum_squared = sum * sum;

    let sqrt = sum_squared.sqrt();

    sqrt
}