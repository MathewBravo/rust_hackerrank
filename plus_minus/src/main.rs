fn plusMinus(arr: &[i32]) {
    let mut neg_count: f32 = 0.0;
    let mut pos_count: f32 = 0.0;
    for i in arr {
        if i < &0 {
            neg_count += 1.0;
        } else if i > &0 {
            pos_count += 1.0;
        }
    }
    let f_size = arr.len() as f32;

    println!("{:.6}", pos_count / f_size);
    println!("{:.6}", neg_count / f_size);
    println!("{:.6}", (f_size - pos_count - neg_count) / f_size);
}

fn main() {
    let test_case = [-4, 3, -9, 0, 4, 1];
    plusMinus(&test_case);
}
