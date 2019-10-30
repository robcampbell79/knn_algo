

pub fn euclid_distance(point1: Vec<Vec<f64>>, point2: Vec<f64>) -> Vec<Vec<f64>> {

    //let mut result = Vec::new();
    let mut results = Vec::new();
    let mut p1: f64 = 0.0;
    let mut p2: f64 = 0.0;
    let mut sum: f64 = 0.0;
    let mut sum_squared: f64 = 0.0;
    //let mut i: usize = 0;

    for i in 0..point1.len() {
        
        p1 = point1[i][0] - point2[0];
        p1 = p1 * p1;

        p2 = point1[i][1] - point2[1];
        p2 = p2 * p2;

        sum = p1 + p2;

        sum_squared = sum.sqrt();

        results.push(vec![sum_squared, point1[i][2]]);
    }

    println!("Pre-sort: {:?}", results);

    let sorted = sort_results(results);

    //results
    sorted
}

fn sort_results(results: Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let mut set = results;

    for i in 0..set.len() {
        let mut min_ind = i;
        for j in i+1..set.len() {
            if set[j][0] < set[i][0] {
                min_ind = j;

                let temp1 = set[min_ind][0];
                let temp2 = set[min_ind][1];
                set[min_ind][0] = set[i][0];
                set[min_ind][1] = set[i][1];
                set[i][0] = temp1;
                set[i][1] = temp2;
            }
        }
    }

    set
}

pub fn predict(set: Vec<Vec<f64>>) -> f64 {

    let male = 0.0;
    let female= 1.0;
    let mut mcount: i32 = 0;
    let mut fcount: i32 = 0;
    let result: f64;

    for i in 0..set.len()-1 {
        if set[i][1] == 0.0 {
            mcount += 1;
        } else {
            fcount += 1;
        }
    }

    if mcount > fcount {
        result = male;
    } else {
        result = female;
    }

    result
}