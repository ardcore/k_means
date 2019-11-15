extern crate k_means;

fn main() {
    let data = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let data_float: Vec<f32> = data.iter().map(|v| *v as f32).collect();
    let result = k_means::solve(&data, 4, 100);
    let result_float = k_means::solve(&data_float, 4, 100);
    println!("result: {:?}", result);
    println!("result_float: {:?}", result_float);
}
