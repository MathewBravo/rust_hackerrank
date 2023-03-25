fn repeatedString(s: &str, n: i64) -> i64 {
    let mut a_count = 0;
    for c in s.chars() {
        if c == 'a' {
            a_count += 1;
        }
    }
    let remainder = usize::try_from(n).expect("Could not get usize from N") % s.len();
    let test = (usize::try_from(n).expect("Could not get usize from N") - remainder) / s.len();
    let mut a_count = a_count * test;
    let remaining_string = &s[0..remainder];
    for c in remaining_string.chars() {
        if c == 'a' {
            a_count += 1;
        }
    }
    i64::try_from(a_count).expect("No i64")
}

fn main() {
    let mut s = "a";
    let n = 1000000000000;
    let result = repeatedString(&s, n);
    println!("{}", result)
}
