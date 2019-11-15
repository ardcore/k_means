extern crate k_means;

fn main() {
    let data = vec![
        1, 2, 3, 4, 7, 8, 9, 10, 13, 14, 15, 17, 18, 19, 20,
    ];
    let data_float: Vec<f32> = data.iter().map(|v| *v as f32).collect();
    let data_neg: Vec<i32> = data.iter().map(|v| { if v % 2 == 0 { *v } else { -*v }}).collect();
    let result = k_means::solve(&data, 4, 150);
    let result_float = k_means::solve(&data_float, 4, 150);
    let result_with_neg = k_means::solve(&data_neg, 4, 150);
    println!("result: {:?}\n", result);
    println!("result_float: {:?}\n", result_float);
    println!("result_with_neg: {:?}\n", result_with_neg);
}
