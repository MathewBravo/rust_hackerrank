fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut most_points = scores[0];
    let mut least_points = scores[0];
    let mut least_counter = 0;
    let mut most_counter = 0;
    for i in scores {
        if i < &least_points {
            least_points = *i;
            least_counter += 1;
        }
        if i > &most_points {
            most_points = *i;
            most_counter += 1;
        }
    }
    let tally: Vec<i32> = vec![most_counter, least_counter];
    tally
}

fn main() {
    let scores = [12, 24, 10, 24];
    let tally = breakingRecords(&scores);
    println!("{}, {}", tally[0], tally[1])
}
