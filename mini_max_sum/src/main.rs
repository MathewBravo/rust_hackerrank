fn miniMaxSum(arr: &[i32]) {
    let mut min = arr[0] as i64;
    let mut max = arr[0] as i64;
    let mut sum = 0 as i64;
    for i in arr {
        let big_i = *i as i64;
        sum += big_i;
        if big_i < min {
            min = big_i;
        }
        if big_i > max {
            max = big_i;
        }
    }
    println!("{} {}", (sum - max), (sum - min))
}
fn main() {
    let arr = [256741038, 623958417, 467905213, 714532089, 938071625];
    miniMaxSum(&arr);
}
