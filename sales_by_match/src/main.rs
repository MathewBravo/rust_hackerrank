use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut socks: HashMap<&i32, i32> = HashMap::new();
    let mut count: i32 = 0;
    for i in ar.iter() {
        socks.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }
    for (_, value) in socks {
        if value % 2 == 0 {
            count += value / 2;
        } else if value > 2 {
            count += (value - 1) / 2
        }
    }
    count
}

fn main() {
    let n = 15;
    let ar = [6, 5, 2, 3, 5, 2, 2, 1, 1, 5, 1, 3, 3, 3, 5];
    let n2 = 10;
    let ar2 = [1, 1, 3, 1, 2, 1, 3, 3, 3, 3];
    let result = sockMerchant(n, &ar);
    let result2 = sockMerchant(n2, &ar2);
    println!("{}", result);
    println!("{}", result2)
}
