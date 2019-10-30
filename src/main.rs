use knn_algo::euclid_distance;
use knn_algo::predict;

fn main() {
    println!("Gonna do knn algorithm here.");

    let p1 = vec![
        vec![73.84701702, 241.89356318, 0.0],
        vec![68.78190405, 162.31047252, 0.0],
        vec![66.1038728, 148.64518257, 1.0],
        vec![64.52718203, 132.68086824, 1.0],
    ];

    // let p2 = vec![76.0, 252.1];
    let p2 = vec![67.0, 158.1];

    let result = euclid_distance(p1, p2);

    println!("{:?}", result);

    let prediction = predict(result);

    println!("Prediction: {}", prediction);
}
